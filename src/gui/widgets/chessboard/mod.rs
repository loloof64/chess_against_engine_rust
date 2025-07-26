use iced::{
    Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Widget, layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
};

pub struct Chessboard;

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
        let background_bounds = Rectangle {
            width: allocated_width,
            height: allocated_width,
            x: allocated_bounds.x,
            y: allocated_bounds.y,
        };

        renderer.fill_quad(
            Quad {
                bounds: background_bounds,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            Color::from_rgb8(120, 71, 145),
        );
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
