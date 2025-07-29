#![windows_subsystem = "windows"]

mod gui;
use std::sync::LazyLock;

use gui::widgets::chessboard::Chessboard;
use iced::{
    Background, Color, Length,
    alignment::{Horizontal, Vertical},
    widget::{Svg, button, column, container, row, svg::Handle},
};

use crate::gui::widgets::chessboard::{self, ChessboardOptionsBuilder};

static SWAP_VERT_BYTES: &[u8] = include_bytes!("swap-vert.svg");
static SWAP_VERT_HANDLE: LazyLock<Handle> = LazyLock::new(|| Handle::from_memory(SWAP_VERT_BYTES));

fn main() -> iced::Result {
    iced::run("Chess against engine", App::update, App::view)
}

#[derive(Debug, Clone)]
enum Message {
    ToggleBoardOrientation,
    UpdatePosition(String),
}

struct App {
    board_reversed: bool,
    board_fen: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            board_reversed: false,
            board_fen: owlchess::Board::initial().as_fen(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleBoardOrientation => self.board_reversed = !self.board_reversed,
            Message::UpdatePosition(new_position) => self.board_fen = new_position,
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
                    .set_position(self.board_fen.to_string())
                    .build(),
                chessboard::MessageProducer {
                    build_update_position: App::build_update_position_message,
                }
            ))
            .center(Length::Fill)
        ]
        .align_x(Horizontal::Center)
        .padding(10)
        .spacing(10)
        .into()
    }

    fn build_update_position_message(new_position: String) -> Message {
        Message::UpdatePosition(new_position)
    }
}
