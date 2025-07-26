//! A chessboard component
mod colors;

pub use colors::ChessboardColors;

use iced::{
    Border, Element, Length, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        svg::Svg,
        widget::Tree,
    },
    widget::svg::Handle,
};
use owlchess::{File, Rank};

pub struct Chessboard {
    colors: ChessboardColors,
    fen: String,
}

impl Chessboard {
    /// Chessboard with default colors.
    pub fn new() -> Self {
        Chessboard {
            colors: ChessboardColors::default(),
            fen: owlchess::Board::initial().as_fen(),
        }
    }

    /// Chessboard with custom colors.
    pub fn new_from_colors(colors: ChessboardColors) -> Self {
        Chessboard {
            colors: colors,
            fen: owlchess::Board::initial().as_fen(),
        }
    }

    /// Chessboard with custom Forsyth-Edwards Notation position.
    pub fn new_from_position(fen: String) -> Self {
        Chessboard {
            colors: ChessboardColors::default(),
            fen: fen,
        }
    }

    /// Chessboard with custom colors and Forsyth-Edwards Notation position.
    pub fn new_from_colors_and_position(colors: ChessboardColors, fen: String) -> Self {
        Chessboard {
            colors: colors,
            fen: fen,
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
                    x: cell_size * (0.5 + col as f32),
                    y: cell_size * (0.5 + row as f32),
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
                let file = col;
                let rank = row;
                let board_logic_cell =
                    board_logic.get2(File::from_index(file), Rank::from_index(rank));
                let is_occupied_cell = board_logic_cell.is_occupied();
                if is_occupied_cell {
                    let piece_bounds = Rectangle {
                        x: cell_size * (0.5 + col as f32),
                        y: cell_size * (0.5 + row as f32),
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
        let image = Chessboard::piece_to_image(piece_type, piece_color);
        let image_handle = Handle::from_memory(image);
        let piece_svg = Svg::new(image_handle);
        renderer.draw_svg(piece_svg, bounds);
    }

    fn piece_to_image(piece_type: owlchess::Piece, piece_color: owlchess::Color) -> &'static [u8] {
        match piece_type {
            owlchess::Piece::Pawn => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_plt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_pdt45.svg"),
            },
            owlchess::Piece::Knight => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_nlt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_ndt45.svg"),
            },
            owlchess::Piece::Bishop => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_blt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_bdt45.svg"),
            },
            owlchess::Piece::Rook => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_rlt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_rdt45.svg"),
            },
            owlchess::Piece::Queen => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_qlt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_qdt45.svg"),
            },
            owlchess::Piece::King => match piece_color {
                owlchess::Color::White => include_bytes!("assets/Chess_klt45.svg"),
                owlchess::Color::Black => include_bytes!("assets/Chess_kdt45.svg"),
            },
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chessboard
where
    Renderer: iced::advanced::Renderer + iced::advanced::svg::Renderer,
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
        _viewport: &Rectangle,
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
    }
}

impl<'a, Message, Renderer> From<Chessboard> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::svg::Renderer,
{
    fn from(widget: Chessboard) -> Self {
        Self::new(widget)
    }
}
