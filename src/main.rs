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
        "Hello, World !".into()
    }
}
