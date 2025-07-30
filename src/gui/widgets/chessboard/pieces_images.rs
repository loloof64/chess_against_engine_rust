use iced::widget::svg::Handle;

pub struct PiecesImages {
    pub white_pawn_handle: Handle,
    pub white_knight_handle: Handle,
    pub white_bishop_handle: Handle,
    pub white_rook_handle: Handle,
    pub white_queen_handle: Handle,
    pub white_king_handle: Handle,
    pub black_pawn_handle: Handle,
    pub black_knight_handle: Handle,
    pub black_bishop_handle: Handle,
    pub black_rook_handle: Handle,
    pub black_queen_handle: Handle,
    pub black_king_handle: Handle,
}

impl PiecesImages {
    pub fn new() -> Self {
        let white_pawn_def = include_bytes!("assets/Chess_plt45.svg");
        let white_pawn_handle = Handle::from_memory(white_pawn_def);

        let white_knight_def = include_bytes!("assets/Chess_nlt45.svg");
        let white_knight_handle = Handle::from_memory(white_knight_def);

        let white_bishop_def = include_bytes!("assets/Chess_blt45.svg");
        let white_bishop_handle = Handle::from_memory(white_bishop_def);

        let white_rook_def = include_bytes!("assets/Chess_rlt45.svg");
        let white_rook_handle = Handle::from_memory(white_rook_def);

        let white_queen_def = include_bytes!("assets/Chess_qlt45.svg");
        let white_queen_handle = Handle::from_memory(white_queen_def);

        let white_king_def = include_bytes!("assets/Chess_klt45.svg");
        let white_king_handle = Handle::from_memory(white_king_def);

        let black_pawn_def = include_bytes!("assets/Chess_pdt45.svg");
        let black_pawn_handle = Handle::from_memory(black_pawn_def);

        let black_knight_def = include_bytes!("assets/Chess_ndt45.svg");
        let black_knight_handle = Handle::from_memory(black_knight_def);

        let black_bishop_def = include_bytes!("assets/Chess_bdt45.svg");
        let black_bishop_handle = Handle::from_memory(black_bishop_def);

        let black_rook_def = include_bytes!("assets/Chess_rdt45.svg");
        let black_rook_handle = Handle::from_memory(black_rook_def);

        let black_queen_def = include_bytes!("assets/Chess_qdt45.svg");
        let black_queen_handle = Handle::from_memory(black_queen_def);

        let black_king_def = include_bytes!("assets/Chess_kdt45.svg");
        let black_king_handle = Handle::from_memory(black_king_def);

        Self {
            white_pawn_handle,
            white_knight_handle,
            white_bishop_handle,
            white_rook_handle,
            white_queen_handle,
            white_king_handle,
            black_pawn_handle,
            black_knight_handle,
            black_bishop_handle,
            black_rook_handle,
            black_queen_handle,
            black_king_handle,
        }
    }
}
