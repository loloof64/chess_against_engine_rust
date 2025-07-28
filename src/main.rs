#![windows_subsystem = "windows"]

mod gui;
use std::sync::LazyLock;

use gui::widgets::chessboard::Chessboard;
use iced::{
    Background, Color, Length,
    alignment::{Horizontal, Vertical},
    widget::{Svg, button, column, container, row, svg::Handle},
};

use crate::gui::widgets::chessboard::ChessboardOptionsBuilder;

static SWAP_VERT_BYTES: &[u8] = include_bytes!("swap-vert.svg");
static SWAP_VERT_HANDLE: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(SWAP_VERT_BYTES));

fn main() -> iced::Result {
    iced::run("Chess against engine", App::update, App::view)
}

#[derive(Debug, Clone)]
enum Message {
    ToggleBoardOrientation,
}

#[derive(Default)]
struct App {
    board_reversed: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBoardOrientation => self.board_reversed = !self.board_reversed,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            container(
                row![
                    button(Svg::new(SWAP_VERT_HANDLE.clone()))
                        .width(50)
                        .height(50)
                        .on_press(Message::ToggleBoardOrientation)
                ]
                .spacing(15.0)
                .align_y(Vertical::Center),
            )
            .padding(5.0)
            .center_x(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(120, 120, 120))),
                ..Default::default()
            }),
            container(Chessboard::new(
                ChessboardOptionsBuilder::new()
                    .set_reversed(self.board_reversed)
                    .set_position(
                        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
                            .to_string()
                    )
                    .build(),
            ))
            .center(Length::Fill)
        ]
        .align_x(Horizontal::Center)
        .padding(10)
        .spacing(10)
        .into()
    }
}
