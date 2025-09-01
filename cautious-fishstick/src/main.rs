use ::iced::theme::Theme;
use iced::{Alignment, Sandbox, Settings};

pub fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

struct RustUI {
    theme: Theme,
    page: Page,
    login_field: LoginField
}

struct LoginField {email: String, password: String}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {Login, Register}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    LoginSubmit,
    Router(String), // change page depending on route
    LoginFIeldCHanged(String, String),
}

impl Sandbox for RustUI {
    type Message = Message;
    fn new() -> Self {
        Self {
            theme: Theme::Dark, // default start theme
            page: Page::Login, // default start page
            login_field: LoginField {
                email: String::new(),
                password: String::new()
            },
        }
    }

    // the app title
    fn title (&self) -> String {
        String::from("Cautious Fishstick - Iced Rust UI")
    }

    fn theme(&self) -> Theme {
        self.theme.clone() // return a copy of the theme.
    }
}