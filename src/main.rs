#![windows_subsystem = "windows"]

mod gui;
use gui::widgets::chessboard::Chessboard;
use iced::{Length, widget::container};

use crate::gui::widgets::chessboard::ChessboardOptions;

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
        container(Chessboard::new(ChessboardOptions::default()))
            .center(Length::Fill)
            .into()
    }
}
