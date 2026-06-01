use std::path::PathBuf;

use iced::{Task, Theme};

use crate::{
    SETTINGS_FILE,
    gui::{
        element::Element,
        message::Message,
        screen::Screen,
        views::{
            main::{MainMessage, MainState},
            settings::{SettingsMessage, SettingsState},
        },
    },
    settings::{Settings, SettingsData},
};

pub struct State {
    settings: Settings,
    screen: Screen,
    main_state: MainState,
    settings_state: SettingsState,
}

impl State {
    pub fn new(config_dir: PathBuf) -> State {
        State {
            settings: Settings::new(config_dir.join(SETTINGS_FILE), SettingsData::default()),
            screen: Screen::Main,
            main_state: MainState::new(),
            settings_state: SettingsState::new(),
        }
    }

    pub fn boot(config_dir: PathBuf) -> (State, Task<Message>) {
        (
            State::new(config_dir.clone()),
            Task::perform(
                Settings::load(config_dir.clone().join(SETTINGS_FILE)),
                |res| match res {
                    Ok(settings) => Message::LoadSettings(settings),
                    Err(_err) => Message::Error,
                },
            ),
        )
    }

    pub fn theme(&self) -> Theme {
        Theme::KanagawaDragon
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Main(message) => match message {
                MainMessage::Settings => self.screen = Screen::Settings,
                message => {
                    return self
                        .main_state
                        .update(&self.settings, message)
                        .map(Message::Main);
                }
            },
            Message::Settings(message) => {
                let settings = &mut self.settings;

                match message {
                    SettingsMessage::ChangeJournalsPath(Some(path)) => {
                        settings.set_journals_path(path);

                        let settings = settings.clone();

                        return Task::perform(async move { settings.save().await }, |_| {
                            Message::SaveSettings
                        });
                    }
                    SettingsMessage::Back => self.screen = Screen::Main,
                    _ => return self.settings_state.update(message).map(Message::Settings),
                }
            }
            Message::Screen(screen) => {
                self.screen = screen;
            }
            Message::LoadSettings(settings) => self.settings = settings,
            _ => (),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => self.main_state.view().map(Message::Main),
            Screen::Settings => self
                .settings_state
                .view(&self.settings)
                .map(Message::Settings),
        }
    }
}
