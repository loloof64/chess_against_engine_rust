use iced::{advanced::svg::Svg, widget::svg::Handle};

pub struct PiecesImages {
    pub white_pawn: Svg,
    pub white_knight: Svg,
    pub white_bishop: Svg,
    pub white_rook: Svg,
    pub white_queen: Svg,
    pub white_king: Svg,
    pub black_pawn: Svg,
    pub black_knight: Svg,
    pub black_bishop: Svg,
    pub black_rook: Svg,
    pub black_queen: Svg,
    pub black_king: Svg,
}

impl PiecesImages {
    pub fn new() -> Self {
        let white_pawn_def = include_bytes!("assets/Chess_plt45.svg");
        let white_pawn_handle = Handle::from_memory(white_pawn_def);
        let white_pawn = Svg::new(white_pawn_handle);

        let white_knight_def = include_bytes!("assets/Chess_nlt45.svg");
        let white_knight_handle = Handle::from_memory(white_knight_def);
        let white_knight = Svg::new(white_knight_handle);

        let white_bishop_def = include_bytes!("assets/Chess_blt45.svg");
        let white_bishop_handle = Handle::from_memory(white_bishop_def);
        let white_bishop = Svg::new(white_bishop_handle);

        let white_rook_def = include_bytes!("assets/Chess_rlt45.svg");
        let white_rook_handle = Handle::from_memory(white_rook_def);
        let white_rook = Svg::new(white_rook_handle);

        let white_queen_def = include_bytes!("assets/Chess_qlt45.svg");
        let white_queen_handle = Handle::from_memory(white_queen_def);
        let white_queen = Svg::new(white_queen_handle);

        let white_king_def = include_bytes!("assets/Chess_klt45.svg");
        let white_king_handle = Handle::from_memory(white_king_def);
        let white_king = Svg::new(white_king_handle);

        let black_pawn_def = include_bytes!("assets/Chess_pdt45.svg");
        let black_pawn_handle = Handle::from_memory(black_pawn_def);
        let black_pawn = Svg::new(black_pawn_handle);

        let black_knight_def = include_bytes!("assets/Chess_ndt45.svg");
        let black_knight_handle = Handle::from_memory(black_knight_def);
        let black_knight = Svg::new(black_knight_handle);

        let black_bishop_def = include_bytes!("assets/Chess_bdt45.svg");
        let black_bishop_handle = Handle::from_memory(black_bishop_def);
        let black_bishop = Svg::new(black_bishop_handle);

        let black_rook_def = include_bytes!("assets/Chess_rdt45.svg");
        let black_rook_handle = Handle::from_memory(black_rook_def);
        let black_rook = Svg::new(black_rook_handle);

        let black_queen_def = include_bytes!("assets/Chess_qdt45.svg");
        let black_queen_handle = Handle::from_memory(black_queen_def);
        let black_queen = Svg::new(black_queen_handle);

        let black_king_def = include_bytes!("assets/Chess_kdt45.svg");
        let black_king_handle = Handle::from_memory(black_king_def);
        let black_king = Svg::new(black_king_handle);

        Self {
            white_pawn: white_pawn,
            white_knight: white_knight,
            white_bishop: white_bishop,
            white_rook: white_rook,
            white_queen: white_queen,
            white_king: white_king,
            black_pawn: black_pawn,
            black_knight: black_knight,
            black_bishop: black_bishop,
            black_rook: black_rook,
            black_queen: black_queen,
            black_king: black_king,
        }
    }
}
