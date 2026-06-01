use std::path::PathBuf;

use iced::{
    Task,
    widget::{button, column, row, text},
};

use crate::{
    dialogs::get_directory,
    gui::{element::Element, views::settings::SettingsMessage::ChangeJournalsPath},
    settings::Settings,
};

pub struct SettingsState {}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChooseJournalsPath,
    ChangeJournalsPath(Option<PathBuf>),
    Back,
}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {}
    }

    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::ChooseJournalsPath => {
                Task::perform(get_directory(), |path| ChangeJournalsPath(path))
            }
            message => Task::done(message),
        }
    }

    pub fn view(&self, settings: &Settings) -> Element<'_, SettingsMessage> {
        column![
            button("Back").on_press(SettingsMessage::Back),
            "Journals directory",
            row![
                button("Change directory").on_press(SettingsMessage::ChooseJournalsPath),
                text(settings.journals_path().display().to_string())
            ]
            .spacing(4)
        ]
        .spacing(8)
        .padding(8)
        .into()
    }
}
