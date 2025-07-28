use crate::gui::widgets::chessboard::ChessboardColors;

#[derive(Debug, Clone)]
pub struct ChessboardOptions {
    pub colors: ChessboardColors,
    pub fen: String,
}

impl Default for ChessboardOptions {
    fn default() -> Self {
        Self {
            colors: ChessboardColors::default(),
            fen: owlchess::Board::initial().as_fen(),
        }
    }
}

pub struct ChessboardOptionsBuilder {
    options: ChessboardOptions,
}

impl ChessboardOptionsBuilder {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            options: ChessboardOptions::default(),
        }
    }

    #[allow(unused)]
    pub fn set_colors(&mut self, colors: ChessboardColors) -> &mut Self {
        self.options.colors = colors;
        self
    }

    #[allow(unused)]
    pub fn set_position(&mut self, fen: String) -> &mut Self {
        self.options.fen = fen;
        self
    }

    #[allow(unused)]
    pub fn build(&self) -> ChessboardOptions {
        self.options.clone()
    }
}
