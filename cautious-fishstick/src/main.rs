use iced::{widget::text, Element};


// boilerplate
#[derive(Debug, Default)]
struct AppState {}

// these are messages coming from the user interface (user interaction) down into the code.
#[derive(Debug)]
enum Message {
    Exit
}

fn update(state: &mut AppState, message: Message) {}

fn view(state: &AppState) -> Element<Message> {
    text("Hello From App").into()
}

fn main() -> iced::Result {
    iced::application("Cautious Fishstick - Iced Rust UI", update, view).run()
}