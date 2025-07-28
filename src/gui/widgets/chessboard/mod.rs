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
        Layout, Text, Widget, layout, mouse,
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

pub struct Chessboard {
    colors: ChessboardColors,
    fen: String,
    reversed: bool,
    images: PiecesImages,
}

impl Chessboard {
    pub fn new(options: ChessboardOptions) -> Self {
        Chessboard {
            colors: options.colors,
            fen: options.fen,
            reversed: options.reversed,
            images: PiecesImages::new(),
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
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chessboard
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
    }
}

impl<'a, Message, Renderer> From<Chessboard> for Element<'a, Message, Theme, Renderer>
where
    Renderer:
        iced::advanced::Renderer + iced::advanced::svg::Renderer + iced::advanced::text::Renderer,
{
    fn from(widget: Chessboard) -> Self {
        Self::new(widget)
    }
}
