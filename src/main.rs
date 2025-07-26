#![windows_subsystem = "windows"]

mod gui;
use gui::widgets::chessboard::Chessboard;
use iced::widget::column;

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
        column![
            Chessboard::new_from_position(
                "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string(),
            ),
            Chessboard::new()
        ]
        .into()
    }
}
