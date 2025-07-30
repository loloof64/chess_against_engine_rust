use iced::{
    Border, Color, Pixels, Point, Rectangle, Shadow,
    advanced::{Text, renderer::Quad, svg::Svg},
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{
        button, svg,
        text::{LineHeight, Shaping, Wrapping},
    },
};
use owlchess::{File, Rank};

use crate::{
    Chessboard,
    gui::widgets::chessboard::{DndData, PendingPromotion},
};

impl<UPM> Chessboard<UPM> {
    pub(crate) fn draw_background(
        &self,
        bounds: Rectangle,
        renderer: &mut impl iced::advanced::Renderer,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: bounds,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            self.colors.background,
        );
    }

    pub(crate) fn draw_cells(
        &self,
        bounds: Rectangle,
        renderer: &mut impl iced::advanced::Renderer,
    ) {
        let common_size = bounds.size().width;
        let cell_size = common_size / 9.0;

        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (col + row) % 2 == 0;
                let color = if is_white_cell {
                    self.colors.white_cell
                } else {
                    self.colors.black_cell
                };
                let cell_bounds = Rectangle {
                    x: bounds.x + cell_size * (0.5 + col as f32),
                    y: bounds.y + cell_size * (0.5 + row as f32),
                    width: cell_size,
                    height: cell_size,
                };

                renderer.fill_quad(
                    Quad {
                        bounds: cell_bounds,
                        border: Border::default(),
                        shadow: Shadow::default(),
                    },
                    color,
                );
            }
        }
    }

    pub(crate) fn draw_pieces(
        &self,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        let common_size = bounds.size().width;
        let cell_size = common_size / 9.0;
        let board_logic = owlchess::Board::from_fen(&self.fen)
            .expect(format!("invalid fen {}", self.fen).as_str());

        for row in 0..8 {
            for col in 0..8 {
                let file = if self.reversed { 7 - col } else { col };
                let rank = if self.reversed { 7 - row } else { row };
                let is_dragged_piece = match self.dnd_data {
                    Some(DndData {
                        start_file,
                        start_rank,
                        ..
                    }) => (file == start_file) && (rank == start_rank),
                    _ => false,
                };
                let is_pending_promotion_piece = match self.pending_promotion {
                    Some(PendingPromotion {
                        start_file,
                        start_rank,
                        ..
                    }) => (file == start_file) && (rank == start_rank),
                    _ => false,
                };
                if is_dragged_piece || is_pending_promotion_piece {
                    continue;
                }
                let board_logic_cell = board_logic.get2(
                    File::from_index(file as usize),
                    Rank::from_index(7 - rank as usize),
                );
                let is_occupied_cell = board_logic_cell.is_occupied();
                if is_occupied_cell {
                    let piece_bounds = Rectangle {
                        x: bounds.x + cell_size * (0.5 + col as f32),
                        y: bounds.y + cell_size * (7.5 - row as f32),
                        width: cell_size,
                        height: cell_size,
                    };
                    let piece_type = board_logic_cell.piece().unwrap();
                    let piece_color = board_logic_cell.color().unwrap();

                    self.draw_single_piece(piece_type, piece_color, piece_bounds, renderer);
                }
            }
        }
    }

    pub(crate) fn draw_single_piece(
        &self,
        piece_type: owlchess::Piece,
        piece_color: owlchess::Color,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        let piece_svg = self.piece_to_svg(piece_type, piece_color);
        renderer.draw_svg(piece_svg, bounds);
    }

    pub(crate) fn piece_to_svg(
        &self,
        piece_type: owlchess::Piece,
        piece_color: owlchess::Color,
    ) -> Svg {
        Svg::new(match piece_type {
            owlchess::Piece::Pawn => match piece_color {
                owlchess::Color::White => self.images.white_pawn_handle.clone(),
                owlchess::Color::Black => self.images.black_pawn_handle.clone(),
            },
            owlchess::Piece::Knight => match piece_color {
                owlchess::Color::White => self.images.white_knight_handle.clone(),
                owlchess::Color::Black => self.images.black_knight_handle.clone(),
            },
            owlchess::Piece::Bishop => match piece_color {
                owlchess::Color::White => self.images.white_bishop_handle.clone(),
                owlchess::Color::Black => self.images.black_bishop_handle.clone(),
            },
            owlchess::Piece::Rook => match piece_color {
                owlchess::Color::White => self.images.white_rook_handle.clone(),
                owlchess::Color::Black => self.images.black_rook_handle.clone(),
            },
            owlchess::Piece::Queen => match piece_color {
                owlchess::Color::White => self.images.white_queen_handle.clone(),
                owlchess::Color::Black => self.images.black_queen_handle.clone(),
            },
            owlchess::Piece::King => match piece_color {
                owlchess::Color::White => self.images.white_king_handle.clone(),
                owlchess::Color::Black => self.images.black_king_handle.clone(),
            },
        })
    }

    pub(crate) fn draw_coordinates(
        &self,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::text::Renderer),
        viewport: &Rectangle,
    ) {
        let common_size = bounds.size().width;
        let cell_size = common_size / 9.0;

        let width = cell_size * 0.4;
        let height = cell_size * 0.48;

        let font = renderer.default_font();

        for col in 0..8 {
            let file = if self.reversed { 7 - col } else { col };
            let letter = (('A' as u8) + file) as char;

            let text_position_1 = Point {
                x: bounds.x + cell_size * (0.855 + col as f32),
                y: bounds.y + cell_size * 0.000_000_01,
            };

            let text_position_2 = Point {
                x: bounds.x + cell_size * (0.855 + col as f32),
                y: bounds.y + cell_size * 8.500_000_01,
            };

            renderer.fill_text(
                Text {
                    wrapping: Wrapping::None,
                    content: format!("{letter}"),
                    bounds: bounds.size(),
                    size: Pixels(width),
                    line_height: LineHeight::Absolute(Pixels(height)),
                    font: font,
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Top,
                    shaping: Shaping::default(),
                },
                text_position_1,
                self.colors.coordinates,
                *viewport,
            );

            renderer.fill_text(
                Text {
                    wrapping: Wrapping::None,
                    content: format!("{letter}"),
                    bounds: bounds.size(),
                    size: Pixels(width),
                    line_height: LineHeight::Absolute(Pixels(height)),
                    font: font,
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Top,
                    shaping: Shaping::default(),
                },
                text_position_2,
                self.colors.coordinates,
                *viewport,
            );
        }

        for row in 0..8 {
            let rank = if self.reversed { 7 - row } else { row };
            let digit = (('1' as u8) + rank) as char;

            let text_position_1 = Point {
                x: bounds.x + cell_size * 0.15,
                y: bounds.y + cell_size * (0.80 + (7 - row) as f32),
            };

            let text_position_2 = Point {
                x: bounds.x + cell_size * 8.65,
                y: bounds.y + cell_size * (0.80 + (7 - row) as f32),
            };

            renderer.fill_text(
                Text {
                    wrapping: Wrapping::None,
                    content: format!("{digit}"),
                    bounds: bounds.size(),
                    size: Pixels(width),
                    line_height: LineHeight::Absolute(Pixels(height)),
                    font: font,
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Top,
                    shaping: Shaping::default(),
                },
                text_position_1,
                self.colors.coordinates,
                *viewport,
            );

            renderer.fill_text(
                Text {
                    wrapping: Wrapping::None,
                    content: format!("{digit}"),
                    bounds: bounds.size(),
                    size: Pixels(width),
                    line_height: LineHeight::Absolute(Pixels(height)),
                    font: font,
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Top,
                    shaping: Shaping::default(),
                },
                text_position_2,
                self.colors.coordinates,
                *viewport,
            );
        }
    }

    pub(crate) fn draw_player_turn(
        &self,
        bounds: Rectangle,
        renderer: &mut impl iced::advanced::Renderer,
    ) {
        let common_size = bounds.size().width;
        let cell_size = common_size / 9.0;

        let board_logic = owlchess::Board::from_fen(&self.fen).expect("invalid fen");
        let is_white_turn = board_logic.side() == owlchess::Color::White;

        let x_factor = if self.reversed { 0.025 } else { 8.5 };
        let y_factor = if self.reversed {
            if is_white_turn { 0.025 } else { 8.5 }
        } else {
            if is_white_turn { 8.5 } else { 0.025 }
        };

        let x = cell_size * x_factor + bounds.x;
        let y = cell_size * y_factor + bounds.y;
        let size = cell_size * 0.5;

        let circle_bounds = Rectangle {
            x,
            y,
            width: size,
            height: size,
        };

        let color = if is_white_turn {
            self.colors.white_turn
        } else {
            self.colors.black_turn
        };

        let border_width = cell_size * 0.05;
        let border_radius = cell_size * 0.5;

        renderer.fill_quad(
            Quad {
                bounds: circle_bounds,
                border: Border {
                    color: Color::BLACK,
                    width: border_width,
                    radius: Radius::new(Pixels(border_radius)),
                },
                shadow: Shadow::default(),
            },
            color,
        );
    }

    pub(crate) fn draw_dragged_piece(
        &self,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        if let Some(dnd_data) = self.dnd_data.clone() {
            let common_size = bounds.size().width;
            let cell_size = common_size / 9.0;
            let half_cell_size = cell_size / 2.0;

            let piece_svg = self.piece_to_svg(dnd_data.piece_type, dnd_data.piece_color);
            let piece_bounds = Rectangle {
                x: dnd_data.location.x - half_cell_size,
                y: dnd_data.location.y - half_cell_size,
                width: cell_size,
                height: cell_size,
            };
            renderer.draw_svg(piece_svg, piece_bounds);
        }
    }

    pub(crate) fn draw_pending_promotion_piece(
        &self,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        if let Some(pending_promotion) = self.pending_promotion.clone() {
            let common_size = bounds.size().width;
            let cell_size = common_size / 9.0;
            let half_cell_size = cell_size / 2.0;

            let piece_svg = self.piece_to_svg(owlchess::Piece::Pawn, pending_promotion.piece_color);
            let piece_bounds = Rectangle {
                x: pending_promotion.location.x - half_cell_size,
                y: pending_promotion.location.y - half_cell_size,
                width: cell_size,
                height: cell_size,
            };
            renderer.draw_svg(piece_svg, piece_bounds);
        }
    }

    pub(crate) fn draw_promotion_selector(
        &self,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        if self.pending_promotion.clone().is_some() {
            let common_size = bounds.size().width;
            let cell_size = common_size / 9.0;
            let half_cells_size = cell_size / 2.0;

            let selector_size = cell_size * 8.0;
            let selector_offset = half_cells_size;
            let selector_bounds = Rectangle {
                x: bounds.x + selector_offset,
                y: bounds.y + selector_offset,
                width: selector_size,
                height: selector_size,
            };
            let selector_background = Color::from_rgba8(125, 125, 125, 0.5);

            renderer.fill_quad(
                Quad {
                    bounds: selector_bounds,
                    ..Default::default()
                },
                selector_background,
            );

            /*

                let queen_button: iced::widget::Button<'_, UPM, iced::Theme, iced::Renderer> = button(
                    svg(self.images.black_queen_handle.clone())
                        .width(cell_size)
                        .height(cell_size),
                )
                .on_press();
            */
        }
    }
}
