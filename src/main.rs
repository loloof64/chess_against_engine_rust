#![windows_subsystem = "windows"]

mod gui;
use gui::widgets::chessboard::Chessboard;
use iced::Color;

use crate::gui::widgets::chessboard::ChessboardColorsBuilder;

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
        let board_colors = ChessboardColorsBuilder::new()
            .set_background(Color::from_rgb(1.0, 0.0, 0.0))
            .build();
        Chessboard::new_from_colors(board_colors).into()
    }
}
