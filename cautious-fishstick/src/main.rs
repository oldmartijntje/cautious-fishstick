use iced::{widget::text, Element};

#[derive(Debug)]

// boilerplate
struct AppState {}

// these are messages coming from the user interface (user interaction) down into the code.
enum Message {
    Exit
}

fn update(state: &AppState, message: Message) {}

fn view(state: &AppState) -> Element<Message> {
    text("Hello From App").into()
}

fn main() {}