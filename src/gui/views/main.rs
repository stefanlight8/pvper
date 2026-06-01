use chrono::{DateTime, Utc};
use iced::{
    Length,
    widget::{button, column, container, row},
};

use crate::{
    frags::Frag,
    gui::{
        element::Element,
        views::plot::{PlotMessage, PlotState},
    },
};

pub struct MainState {
    frags: Vec<Frag>,
    plot: PlotState,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Scan,
    Settings,
    Plot(PlotMessage),
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            frags: Vec::new(),
            plot: PlotState::new(),
        }
    }

    pub fn update(&mut self, message: MainMessage) {
        match message {
            MainMessage::Plot(message) => self.plot.update(message),
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        row![
            column!("frags").padding(6).width(Length::Fill),
            column![
                container(self.plot.view().map(MainMessage::Plot))
                    .padding(6)
                    .height(Length::Fill),
                container("statistics").padding(6).height(Length::Fill),
                container(
                    row![
                        button("Scan").on_press(MainMessage::Scan),
                        button("Settings").on_press(MainMessage::Settings)
                    ]
                    .spacing(6)
                )
                .padding(6)
                .width(Length::Fill)
            ]
            .width(Length::Fill)
        ]
        .padding(6)
        .into()
    }
}
