use iced::Task;

use crate::gui::{
    element::Element,
    message::Message,
    screen::Screen,
    views::{
        main::{MainMessage, MainState},
        settings::SettingsState,
    },
};

pub struct State {
    main: MainState,
    settings: SettingsState,
    screen: Screen,
}

impl State {
    pub fn new() -> State {
        State {
            main: MainState::new(),
            settings: SettingsState::new(),
            screen: Screen::Main,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Main(message) => match message {
                MainMessage::Settings => self.screen = Screen::Settings,
                message => self.main.update(message),
            },
            Message::Settings(_) => (),
            Message::Screen(screen) => {
                self.screen = screen;
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => self.main.view().map(Message::Main),
            Screen::Settings => self.settings.view().map(Message::Settings),
        }
    }
}
