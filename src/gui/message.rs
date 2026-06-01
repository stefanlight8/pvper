use crate::{
    gui::{
        screen::Screen,
        views::{main::MainMessage, settings::SettingsMessage},
    },
    settings::Settings,
};

#[derive(Debug, Clone)]
pub enum Message {
    Main(MainMessage),
    Settings(SettingsMessage),
    Screen(Screen),
    LoadSettings(Settings),
    SaveSettings,
    Error,
}
