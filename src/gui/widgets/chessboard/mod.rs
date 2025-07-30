//! A chessboard component
mod colors;
mod drawing;
mod event_handling;
mod options;
mod pieces_images;

pub use colors::ChessboardColors;
#[allow(unused)]
pub use options::{ChessboardOptions, ChessboardOptionsBuilder};

use iced::{
    Element, Length, Point, Rectangle, Size, Theme,
    advanced::{
        Layout, Widget,
        graphics::core::event,
        layout, mouse,
        renderer::{self},
        widget::Tree,
    },
};

use crate::gui::widgets::chessboard::pieces_images::PiecesImages;

#[derive(Debug, Clone)]
struct DndData {
    start_file: u8,
    start_rank: u8,
    location: Point,
    piece_type: owlchess::Piece,
    piece_color: owlchess::Color,
}

#[derive(Debug, Clone)]
struct PendingPromotion {
    start_file: u8,
    start_rank: u8,
    end_file: u8,
    end_rank: u8,
    location: Point,
    piece_color: owlchess::Color,
    queen_button_bounds: Rectangle,
    rook_button_bounds: Rectangle,
    bishop_button_bounds: Rectangle,
    knight_button_bounds: Rectangle,
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}

/// The builders for the messages the chessboard
/// component will produce.
/// UPM generic stands for UpdatePositionMessage
#[derive(Debug, Clone)]
pub struct MessageProducer<UPM> {
    pub build_update_position: fn(String) -> UPM,
}

/// A chessboard component
/// UPM generic stands for UpdatePositionMessage
pub struct Chessboard<UPM> {
    colors: ChessboardColors,
    fen: String,
    reversed: bool,
    images: PiecesImages,
    dnd_data: Option<DndData>,
    pending_promotion: Option<PendingPromotion>,
    messages_producer: MessageProducer<UPM>,
}

impl<UPM> Chessboard<UPM> {
    pub fn new(options: ChessboardOptions, messages_producer: MessageProducer<UPM>) -> Self {
        Chessboard {
            colors: options.colors,
            fen: options.fen,
            reversed: options.reversed,
            images: PiecesImages::new(),
            dnd_data: None,
            pending_promotion: None,
            messages_producer,
        }
    }

    fn get_file_and_rank(&self, position: Point, bounds: Rectangle) -> (i8, i8) {
        let common_size = bounds.size().width;
        let cell_size = common_size / 9.0;
        let half_cell_size = cell_size / 2.0;

        let col = ((position.x - half_cell_size) / cell_size).floor() as i8;
        let row = ((position.y - half_cell_size) / cell_size).floor() as i8;

        let file = if self.reversed { 7 - col } else { col };
        let rank = if self.reversed { row } else { 7 - row };

        (file, rank)
    }

    fn in_cell_bounds(file: i8, rank: i8) -> bool {
        file >= 0 && file < 8 && rank >= 0 && rank < 8
    }

    fn get_uci_move(
        start_file: u8,
        start_rank: u8,
        end_file: u8,
        end_rank: u8,
        promotion_piece: Option<PromotionPiece>,
    ) -> String {
        let start_file = ('a' as u8 + start_file) as char;
        let start_rank = ('1' as u8 + start_rank) as char;
        let end_file = ('a' as u8 + end_file) as char;
        let end_rank = ('1' as u8 + end_rank) as char;
        let promotion_piece = match promotion_piece {
            Some(PromotionPiece::Queen) => "q",
            Some(PromotionPiece::Rook) => "r",
            Some(PromotionPiece::Bishop) => "b",
            Some(PromotionPiece::Knight) => "n",
            None => "",
        };
        format!("{start_file}{start_rank}{end_file}{end_rank}{promotion_piece}")
    }

    fn commit_promotion(
        &mut self,
        piece: PromotionPiece,
        shell: &mut iced::advanced::Shell<'_, UPM>,
    ) {
        if let Some(pending_promotion) = self.pending_promotion.clone() {
            let board_logic = owlchess::Board::from_fen(&self.fen).expect("invalid fen");

            let start_file = pending_promotion.start_file;
            let start_rank = pending_promotion.start_rank;
            let end_file = pending_promotion.end_file;
            let end_rank = pending_promotion.end_rank;
            let matching_legal_move = Chessboard::<UPM>::get_uci_move(
                start_file,
                start_rank,
                end_file,
                end_rank,
                Some(piece),
            );
            ///////////////////////////////////
            println!("{matching_legal_move}");
            //////////////////////////////////
            let matching_legal_move =
                owlchess::Move::from_uci_legal(matching_legal_move.as_str(), &board_logic);
            if let Ok(matching_legal_move) = matching_legal_move {
                let resulting_board_logic = board_logic.make_move(matching_legal_move);
                if let Ok(resulting_board_logic) = resulting_board_logic {
                    let new_fen = resulting_board_logic.as_fen();
                    let update_message = (self.messages_producer.build_update_position)(new_fen);
                    self.dnd_data = None;
                    shell.publish(update_message);
                }
            }
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chessboard<Message>
where
    Renderer:
        iced::advanced::Renderer + iced::advanced::svg::Renderer + iced::advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max_size = limits.max();
        let max_width = max_size.width;
        let max_height = max_size.height;

        let common_size = max_width.min(max_height);

        layout::Node::new([common_size, common_size].into())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let allocated_bounds = layout.bounds();
        let allocated_size = allocated_bounds.size();
        let allocated_width = allocated_size.width;
        let bounds = Rectangle {
            width: allocated_width,
            height: allocated_width,
            x: allocated_bounds.x,
            y: allocated_bounds.y,
        };

        self.draw_background(bounds, renderer);
        self.draw_cells(bounds, renderer);
        self.draw_pieces(bounds, renderer);
        self.draw_coordinates(bounds, renderer, viewport);
        self.draw_player_turn(bounds, renderer);
        self.draw_dragged_piece(bounds, renderer);
        self.draw_pending_promotion_piece(bounds, renderer);
        self.draw_promotion_selector(bounds, renderer);
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                self.handle_button_pressed(event, layout, cursor, shell);
                event::Status::Captured
            }
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                self.handle_button_released(event, layout, cursor, shell);
                event::Status::Captured
            }
            iced::Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                self.handle_mouse_moved(event, layout, cursor);
                event::Status::Captured
            }
            _ => event::Status::Ignored,
        }
    }
}

impl<'a, Message: 'a, Renderer> From<Chessboard<Message>> for Element<'a, Message, Theme, Renderer>
where
    Renderer:
        iced::advanced::Renderer + iced::advanced::svg::Renderer + iced::advanced::text::Renderer,
{
    fn from(widget: Chessboard<Message>) -> Self {
        Self::new(widget)
    }
}
