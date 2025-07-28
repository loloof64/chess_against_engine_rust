//! A chessboard component
mod colors;
mod options;
mod pieces_images;

pub use colors::ChessboardColors;
#[allow(unused)]
pub use options::{ChessboardOptions, ChessboardOptionsBuilder};

use iced::{
    Border, Color, Element, Length, Pixels, Point, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Text, Widget,
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        svg::Svg,
        widget::Tree,
    },
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::text::{LineHeight, Shaping, Wrapping},
};
use owlchess::{File, Rank};

use crate::gui::widgets::chessboard::pieces_images::PiecesImages;

#[derive(Debug, Clone)]
struct DndData {
    start_file: u8,
    start_rank: u8,
    location: Point,
    piece_type: owlchess::Piece,
    piece_color: owlchess::Color,
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
            messages_producer,
        }
    }

    fn draw_background(&self, bounds: Rectangle, renderer: &mut impl iced::advanced::Renderer) {
        renderer.fill_quad(
            Quad {
                bounds: bounds,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            self.colors.background,
        );
    }

    fn draw_cells(&self, bounds: Rectangle, renderer: &mut impl iced::advanced::Renderer) {
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

    fn draw_pieces(
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
                let board_logic_cell =
                    board_logic.get2(File::from_index(file), Rank::from_index(rank));
                let is_occupied_cell = board_logic_cell.is_occupied();
                if is_occupied_cell {
                    let piece_bounds = Rectangle {
                        x: bounds.x + cell_size * (0.5 + col as f32),
                        y: bounds.y + cell_size * (0.5 + row as f32),
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

    fn draw_single_piece(
        &self,
        piece_type: owlchess::Piece,
        piece_color: owlchess::Color,
        bounds: Rectangle,
        renderer: &mut (impl iced::advanced::Renderer + iced::advanced::svg::Renderer),
    ) {
        let piece_svg = self.piece_to_svg(piece_type, piece_color);
        renderer.draw_svg(piece_svg, bounds);
    }

    fn piece_to_svg(&self, piece_type: owlchess::Piece, piece_color: owlchess::Color) -> Svg {
        match piece_type {
            owlchess::Piece::Pawn => match piece_color {
                owlchess::Color::White => self.images.white_pawn.clone(),
                owlchess::Color::Black => self.images.black_pawn.clone(),
            },
            owlchess::Piece::Knight => match piece_color {
                owlchess::Color::White => self.images.white_knight.clone(),
                owlchess::Color::Black => self.images.black_knight.clone(),
            },
            owlchess::Piece::Bishop => match piece_color {
                owlchess::Color::White => self.images.white_bishop.clone(),
                owlchess::Color::Black => self.images.black_bishop.clone(),
            },
            owlchess::Piece::Rook => match piece_color {
                owlchess::Color::White => self.images.white_rook.clone(),
                owlchess::Color::Black => self.images.black_rook.clone(),
            },
            owlchess::Piece::Queen => match piece_color {
                owlchess::Color::White => self.images.white_queen.clone(),
                owlchess::Color::Black => self.images.black_queen.clone(),
            },
            owlchess::Piece::King => match piece_color {
                owlchess::Color::White => self.images.white_king.clone(),
                owlchess::Color::Black => self.images.black_king.clone(),
            },
        }
    }

    fn draw_coordinates(
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

    fn draw_player_turn(&self, bounds: Rectangle, renderer: &mut impl iced::advanced::Renderer) {
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

    fn draw_dragged_piece(
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

    fn handle_button_pressed(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        if let iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            // Position relative to the component
            let position = cursor.position_in(layout.bounds());
            if let Some(position) = position {
                let (file, rank) = self.get_file_and_rank(position, layout.bounds());
                if Chessboard::<UPM>::in_cell_bounds(file, rank) {
                    let file = file as u8;
                    let rank = rank as u8;
                    let board_logic = owlchess::Board::from_fen(&self.fen).expect("invalid fen");
                    let matching_cell = board_logic.get2(
                        owlchess::File::from_index(file as usize),
                        owlchess::Rank::from_index(7 - rank as usize),
                    );
                    let piece_color = matching_cell.color();
                    let piece_type = matching_cell.piece();
                    let dnd_position = cursor.position_over(layout.bounds());
                    if let Some(piece_color) = piece_color
                        && let Some(piece_type) = piece_type
                        && let Some(dnd_position) = dnd_position
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

    fn handle_button_released(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        shell: &mut iced::advanced::Shell<'_, UPM>,
    ) {
        if let iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            // Position relative to the component
            let position = cursor.position_in(layout.bounds());
            if let Some(position) = position
                && self.dnd_data.is_some()
            {
                let (file, rank) = self.get_file_and_rank(position, layout.bounds());
                if Chessboard::<UPM>::in_cell_bounds(file, rank) {
                    let dnd_data_clone = self.dnd_data.clone().unwrap();
                    let start_file = dnd_data_clone.start_file as u8;
                    let start_rank = dnd_data_clone.start_rank as u8;
                    let end_file = file as u8;
                    let end_rank = rank as u8;

                    let board_logic = owlchess::Board::from_fen(&self.fen).expect("invalid fen");
                    let matching_move =
                        Chessboard::<UPM>::get_uci_move(start_file, start_rank, end_file, end_rank);
                    let matching_move =
                        owlchess::Move::from_uci_legal(matching_move.as_str(), &board_logic);
                    if let Ok(matching_move) = matching_move {
                        let matching_move = board_logic.make_move(matching_move);
                        if let Ok(board_logic) = matching_move {
                            let new_fen = board_logic.as_fen();
                            let update_message =
                                (self.messages_producer.build_update_position)(new_fen);
                            shell.publish(update_message);
                        }
                    }

                    self.dnd_data = None;
                } else {
                    self.dnd_data = None;
                }
            } else {
                self.dnd_data = None;
            }
        }
    }

    fn handle_mouse_moved(
        &mut self,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
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

    fn get_uci_move(start_file: u8, start_rank: u8, end_file: u8, end_rank: u8) -> String {
        let start_file = ('a' as u8 + start_file) as char;
        let start_rank = ('1' as u8 + start_rank) as char;
        let end_file = ('a' as u8 + end_file) as char;
        let end_rank = ('1' as u8 + end_rank) as char;
        format!("{start_file}{start_rank}{end_file}{end_rank}")
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
                self.handle_button_pressed(event, layout, cursor);
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
