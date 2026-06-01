use crate::gui::{
    screen::Screen,
    views::{main::MainMessage, settings::SettingsMessage},
};

#[derive(Debug, Clone)]
pub enum Message {
    Main(MainMessage),
    Settings(SettingsMessage),
    Screen(Screen),
}
