use iced::{
    Rectangle,
    advanced::{Layout, mouse},
};

use crate::{
    Chessboard,
    gui::widgets::chessboard::{DndData, PendingPromotion, PromotionPiece},
};

impl<UPM> Chessboard<UPM> {
    pub(crate) fn handle_button_pressed(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        shell: &mut iced::advanced::Shell<'_, UPM>,
    ) {
        if let iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            // Position relative to the component
            let overlapping_board_position = cursor.position_in(layout.bounds());
            if let Some(position) = overlapping_board_position {
                if let Some(pending_promotion) = self.pending_promotion.clone() {
                    let overlapping_queen_button_position =
                        cursor.position_in(pending_promotion.queen_button_bounds);
                    let overlapping_rook_button_position =
                        cursor.position_in(pending_promotion.rook_button_bounds);
                    let overlapping_bishop_button_position =
                        cursor.position_in(pending_promotion.bishop_button_bounds);
                    let overlapping_knight_button_position =
                        cursor.position_in(pending_promotion.knight_button_bounds);

                    if overlapping_queen_button_position.is_some() {
                        self.commit_promotion(PromotionPiece::Queen, shell);
                        self.pending_promotion = None;
                    } else if overlapping_rook_button_position.is_some() {
                        self.commit_promotion(PromotionPiece::Rook, shell);
                        self.pending_promotion = None;
                    } else if overlapping_bishop_button_position.is_some() {
                        self.commit_promotion(PromotionPiece::Bishop, shell);
                        self.pending_promotion = None;
                    } else if overlapping_knight_button_position.is_some() {
                        self.commit_promotion(PromotionPiece::Knight, shell);
                        self.pending_promotion = None;
                    }
                } else {
                    let (file, rank) = self.get_file_and_rank(position, layout.bounds());
                    if Chessboard::<UPM>::in_cell_bounds(file, rank) {
                        let file = file as u8;
                        let rank = rank as u8;
                        let board_logic =
                            owlchess::Board::from_fen(&self.fen).expect("invalid fen");
                        let matching_cell = board_logic.get2(
                            owlchess::File::from_index(file as usize),
                            owlchess::Rank::from_index(7 - rank as usize),
                        );
                        let piece_color = matching_cell.color();
                        let piece_type = matching_cell.piece();
                        let dnd_position = cursor.position_over(layout.bounds());
                        let is_white_turn = board_logic.side() == owlchess::Color::White;
                        if let Some(piece_color) = piece_color
                        && let Some(piece_type) = piece_type
                        && let Some(dnd_position) = dnd_position
                        // is it our piece ?
                        && is_white_turn == (piece_color == owlchess::Color::White)
                        {
                            self.dnd_data = Some(DndData {
                                start_file: file,
                                start_rank: rank,
                                location: dnd_position,
                                piece_color,
                                piece_type,
                            });
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn handle_button_released(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        shell: &mut iced::advanced::Shell<'_, UPM>,
    ) {
        let is_pending_promotion = self.pending_promotion.is_some();
        if is_pending_promotion {
            return;
        }
        if let iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            // Position relative to the component
            let overlapping_board_position = cursor.position_in(layout.bounds());
            if let Some(position) = overlapping_board_position {
                if self.dnd_data.is_some() {
                    let (file, rank) = self.get_file_and_rank(position, layout.bounds());
                    if Chessboard::<UPM>::in_cell_bounds(file, rank) {
                        let dnd_data_clone = self.dnd_data.clone().unwrap();
                        let start_file = dnd_data_clone.start_file as u8;
                        let start_rank = dnd_data_clone.start_rank as u8;
                        let end_file = file as u8;
                        let end_rank = rank as u8;

                        let board_logic =
                            owlchess::Board::from_fen(&self.fen).expect("invalid fen");
                        let promotion_move_test = Chessboard::<UPM>::get_uci_move(
                            start_file,
                            start_rank,
                            end_file,
                            end_rank,
                            Some(PromotionPiece::Queen),
                        );
                        let promotion_move_test = owlchess::Move::from_uci_legal(
                            promotion_move_test.as_str(),
                            &board_logic,
                        );
                        let is_promotion_move = match promotion_move_test {
                            Ok(promotion_move) => promotion_move.kind().promote().is_some(),
                            _ => false,
                        };

                        if is_promotion_move {
                            let is_white_turn = board_logic.side() == owlchess::Color::White;
                            let piece_color = if is_white_turn {
                                owlchess::Color::White
                            } else {
                                owlchess::Color::Black
                            };
                            let dnd_data_clone = self.dnd_data.clone().unwrap();
                            let location = dnd_data_clone.location;
                            let start_file = dnd_data_clone.start_file;
                            let start_rank = dnd_data_clone.start_rank;

                            self.dnd_data = None;
                            let bounds = layout.bounds();
                            let cell_size = bounds.size().width / 9.0;
                            let queen_button_bounds = Rectangle {
                                x: bounds.x + cell_size * 1.0,
                                y: bounds.y + cell_size * 4.0,
                                width: cell_size,
                                height: cell_size,
                            };
                            let rook_button_bounds = Rectangle {
                                x: bounds.x + cell_size * 3.0,
                                y: bounds.y + cell_size * 4.0,
                                width: cell_size,
                                height: cell_size,
                            };
                            let bishop_button_bounds = Rectangle {
                                x: bounds.x + cell_size * 5.0,
                                y: bounds.y + cell_size * 4.0,
                                width: cell_size,
                                height: cell_size,
                            };
                            let knight_button_bounds = Rectangle {
                                x: bounds.x + cell_size * 7.0,
                                y: bounds.y + cell_size * 4.0,
                                width: cell_size,
                                height: cell_size,
                            };
                            self.pending_promotion = Some(PendingPromotion {
                                piece_color,
                                location,
                                start_file,
                                start_rank,
                                end_file,
                                end_rank,
                                queen_button_bounds,
                                rook_button_bounds,
                                bishop_button_bounds,
                                knight_button_bounds,
                            });
                        } else {
                            let matching_legal_move = Chessboard::<UPM>::get_uci_move(
                                start_file, start_rank, end_file, end_rank, None,
                            );
                            let matching_legal_move = owlchess::Move::from_uci_legal(
                                matching_legal_move.as_str(),
                                &board_logic,
                            );
                            if let Ok(matching_legal_move) = matching_legal_move {
                                let resulting_board_logic =
                                    board_logic.make_move(matching_legal_move);
                                if let Ok(resulting_board_logic) = resulting_board_logic {
                                    let new_fen = resulting_board_logic.as_fen();
                                    let update_message =
                                        (self.messages_producer.build_update_position)(new_fen);
                                    self.dnd_data = None;
                                    shell.publish(update_message);
                                }
                            } else {
                                self.dnd_data = None;
                            }
                        }
                    } else {
                        self.dnd_data = None;
                    }
                }
            } else {
                self.dnd_data = None;
            }
        }
    }

    pub(crate) fn handle_mouse_moved(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let is_pending_promotion = self.pending_promotion.is_some();
        if is_pending_promotion {
            return;
        }
        if let iced::Event::Mouse(mouse::Event::CursorMoved { position: _ }) = event {
            // Position relative to the component
            let position = cursor.position_in(layout.bounds());
            if let Some(position) = position
                && self.dnd_data.is_some()
            {
                let (file, rank) = self.get_file_and_rank(position, layout.bounds());
                let dnd_position = cursor.position_over(layout.bounds());
                if Chessboard::<UPM>::in_cell_bounds(file, rank)
                    && let Some(dnd_position) = dnd_position
                {
                    self.dnd_data = Some(DndData {
                        location: dnd_position,
                        ..self.dnd_data.clone().unwrap()
                    });
                }
            }
        }
    }
}
