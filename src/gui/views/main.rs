use {
    crate::{
        frags::Frag,
        gui::{
            element::Element,
            views::{
                plot::{PlotMessage, PlotState},
                statistics::Statistics,
            },
            widget::frag::frag,
        },
        journals::{get_journals, scan_journals},
        settings::Settings,
    },
    iced::{
        Length, Padding, Task,
        widget::{button, column, container, row, scrollable},
    },
    std::path::PathBuf,
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
    Frags(Vec<Frag>),
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

    pub fn update(&mut self, settings: &Settings, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Scan => {
                return Task::perform(get_journals(settings.journals_path()), |res| {
                    match res {
                        Ok(mut journals) => {
                            journals.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

                            MainMessage::Journals(journals)
                        }
                        Err(err) => {
                            tracing::error!("failed to get journals: {}", err);

                            MainMessage::Error
                        } // TODO: error
                    }
                });
            }
            MainMessage::Journals(journals) => {
                return Task::stream(scan_journals(journals)).map(|res| match res {
                    Ok(frag) => MainMessage::Frags(frag),
                    Err(err) => {
                        tracing::error!("failed to scan journals: {}", err);

                        MainMessage::Error
                    } // TODO: error
                });
            }
            MainMessage::Frags(frags) => {
                tracing::debug!("received frags: {:?}", frags);

                if frags.is_empty() {
                    return Task::none();
                }

                self.statistics.frags(frags.clone());
                self.plot.frags(frags.clone());
                self.frags.extend(frags);
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
