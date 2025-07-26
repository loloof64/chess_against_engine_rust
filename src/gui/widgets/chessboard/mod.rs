//! A chessboard component
mod colors;

pub use colors::{ChessboardColors, ChessboardColorsBuilder};

use iced::{
    Border, Element, Length, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
};

pub struct Chessboard {
    colors: ChessboardColors,
}

impl Chessboard {
    /// Chessboard with default colors.
    pub fn new() -> Self {
        Chessboard {
            colors: ChessboardColors::default(),
        }
    }

    /// Chessboard with custom colors.
    pub fn new_from_colors(colors: ChessboardColors) -> Self {
        Chessboard { colors: colors }
    }

    fn draw_background(&self, renderer: &mut impl iced::advanced::Renderer, bounds: Rectangle) {
        renderer.fill_quad(
            Quad {
                bounds: bounds,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            self.colors.background,
        );
    }

    fn draw_cells(&self, renderer: &mut impl iced::advanced::Renderer, bounds: Rectangle) {
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
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chessboard
where
    Renderer: iced::advanced::Renderer,
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

        self.draw_background(renderer, bounds);
        self.draw_cells(renderer, bounds);
    }
}

impl<'a, Message, Renderer> From<Chessboard> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn from(widget: Chessboard) -> Self {
        Self::new(widget)
    }
}
