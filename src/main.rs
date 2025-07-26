#![windows_subsystem = "windows"]

mod gui;
use gui::widgets::chessboard::Chessboard;

fn main() -> iced::Result {
    iced::run("Chess against engine", App::update, App::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        /*
        let board_colors = ChessboardColorsBuilder::new()
            .set_background(Color::from_rgb(1.0, 0.0, 0.0))
            .set_white_cell(Color::from_rgb8(0, 255, 0))
            .set_black_cell(Color::from_rgb8(0, 0, 255))
            .build();
        Chessboard::new_from_colors(board_colors).into()
        */
        Chessboard::new_from_position(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string(),
        )
        .into()
    }
}
