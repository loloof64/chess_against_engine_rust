//! Utilities for customizing the colors of a [`super::Chessboard`].
use iced::Color;

/// Customize the colors of a [`super::Chessboard`].
#[derive(Debug, Clone)]
pub struct ChessboardColors {
    pub background: Color,
}

/// Defaults colors for a [`super::Chessboard`].
impl Default for ChessboardColors {
    fn default() -> Self {
        Self {
            background: Color::from_rgb8(120, 71, 145),
        }
    }
}

/// Builds a [`ChessboardColors`] and lets you override the colors you need.
pub struct ChessboardColorsBuilder {
    colors: ChessboardColors,
}

impl ChessboardColorsBuilder {
    pub fn new() -> Self {
        ChessboardColorsBuilder {
            colors: ChessboardColors::default(),
        }
    }

    pub fn build(&self) -> ChessboardColors {
        self.colors.clone()
    }

    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.colors.background = color;
        self
    }
}
