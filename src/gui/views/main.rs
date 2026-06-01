use iced::{
    Length, Padding, Task,
    widget::{button, column, container, row, scrollable},
};
use std::path::PathBuf;

use crate::{
    frags::{Frag, scan_journal},
    gui::{
        element::Element,
        views::{
            plot::{PlotMessage, PlotState},
            statistics::Statistics,
        },
        widget::frag::frag,
    },
    journals::get_journals,
};

pub struct MainState {
    frags: Vec<Frag>,
    statistics: Statistics,
    plot: PlotState,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Scan,
    Settings,
    Journals(Vec<PathBuf>),
    Frag(Frag),
    Plot(PlotMessage),
    Error,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            frags: Vec::new(),
            statistics: Statistics::new(),
            plot: PlotState::new(),
        }
    }

    pub fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Scan => {
                return Task::perform(get_journals("test"), |res| {
                    match res {
                        Ok(journals) => MainMessage::Journals(journals),
                        Err(err) => MainMessage::Error, // TODO: error
                    }
                });
            }
            MainMessage::Journals(journals) => {
                return Task::batch(journals.iter().cloned().map(|path| {
                    Task::stream(scan_journal(path)).map(|res| match res {
                        Ok(frag) => MainMessage::Frag(frag),
                        Err(err) => MainMessage::Error, // TODO: error
                    })
                }));
            }
            MainMessage::Frag(frag) => {
                self.statistics.frag(&frag);
                self.plot.frag(&frag);

                let idx = self
                    .frags
                    .binary_search_by(|f| frag.timestamp.cmp(&f.timestamp))
                    .unwrap_or_else(|e| e);
                self.frags.insert(idx, frag);
            }
            MainMessage::Plot(message) => self.plot.update(message),
            _ => (),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        row![
            scrollable(
                column(self.frags.iter().map(|entry| frag(entry).into()))
                    .padding(Padding {
                        top: 6.,
                        bottom: 6.,
                        left: 6.,
                        right: 18., // because of scrollbar
                    })
                    .spacing(4)
            )
            .width(Length::Fill),
            column![
                container(self.plot.view().map(MainMessage::Plot))
                    .padding(6)
                    .height(Length::Fill),
                scrollable(
                    container(self.statistics.view())
                        .padding(6)
                        .height(Length::Fill)
                ),
                container(
                    row![
                        button("Scan").on_press_maybe(if self.frags.is_empty() {
                            Some(MainMessage::Scan)
                        } else {
                            None
                        }),
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
