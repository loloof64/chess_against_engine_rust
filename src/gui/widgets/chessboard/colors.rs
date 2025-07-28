//! Utilities for customizing the colors of a [`super::Chessboard`].
use iced::Color;

/// Customize the colors of a [`super::Chessboard`].
#[derive(Debug, Clone)]
pub struct ChessboardColors {
    pub background: Color,
    pub white_cell: Color,
    pub black_cell: Color,
    pub coordinates: Color,
    pub white_turn: Color,
    pub black_turn: Color,
}

/// Defaults colors for a [`super::Chessboard`].
impl Default for ChessboardColors {
    fn default() -> Self {
        Self {
            background: Color::from_rgb8(120, 71, 145),  // purple
            white_cell: Color::from_rgb8(255, 222, 173), // navajowhite
            black_cell: Color::from_rgb8(205, 133, 63),  // peru
            coordinates: Color::from_rgb8(255, 215, 0),  // gold
            white_turn: Color::WHITE,
            black_turn: Color::BLACK,
        }
    }
}

/// Builds a [`ChessboardColors`] and lets you override the colors you need.
pub struct ChessboardColorsBuilder {
    #[allow(unused)]
    colors: ChessboardColors,
}

impl ChessboardColorsBuilder {
    #[allow(unused)]
    pub fn new() -> Self {
        ChessboardColorsBuilder {
            colors: ChessboardColors::default(),
        }
    }

    #[allow(unused)]
    pub fn build(&self) -> ChessboardColors {
        self.colors.clone()
    }

    #[allow(unused)]
    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.colors.background = color;
        self
    }

    #[allow(unused)]
    pub fn set_white_cell(&mut self, color: Color) -> &mut Self {
        self.colors.white_cell = color;
        self
    }

    #[allow(unused)]
    pub fn set_black_cell(&mut self, color: Color) -> &mut Self {
        self.colors.black_cell = color;
        self
    }

    #[allow(unused)]
    pub fn set_coordinates(&mut self, color: Color) -> &mut Self {
        self.colors.coordinates = color;
        self
    }

    #[allow(unused)]
    pub fn set_white_turn(&mut self, color: Color) -> &mut Self {
        self.colors.white_turn = color;
        self
    }

    #[allow(unused)]
    pub fn set_black_turn(&mut self, color: Color) -> &mut Self {
        self.colors.black_turn = color;
        self
    }
}
