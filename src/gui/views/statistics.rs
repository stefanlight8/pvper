use {
    crate::{
        frags::{Frag, Target},
        gui::element::{Column, Element},
    },
    chrono::{DateTime, Utc},
    iced::{
        Length,
        widget::{column, container, row, text},
    },
    std::collections::HashSet,
};

fn stat_card<'a, Message: 'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(column![text(label).size(12), text(value).size(22)].spacing(4))
        .padding(10)
        .width(Length::Fill)
        .into()
}

#[derive(Debug, Default)]
pub struct Statistics {
    frags: Vec<Frag>,

    kills: usize,
    deaths: usize,

    kd: f64,
    unique_players: usize,

    total_frags: usize,

    first_frag: Option<DateTime<Utc>>,
    last_frag: Option<DateTime<Utc>>,
}

impl Statistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view<'a, Message: 'a>(&'a self) -> Column<'a, Message> {
        column![
            row![
                stat_card("Total fights", format!("{:.2}", self.total_frags)),
                stat_card("Players", self.unique_players.to_string()),
            ]
            .spacing(8),
            row![
                stat_card("Kills", self.kills.to_string()),
                stat_card("Deaths", self.deaths.to_string()),
            ]
            .spacing(8),
            row![
                stat_card(
                    "First fight",
                    self.first_frag
                        .map(|t| t.format("%d %b %Y").to_string())
                        .unwrap_or_else(|| "-".into()),
                ),
                stat_card(
                    "Last fight",
                    self.last_frag
                        .map(|t| t.format("%d %b %Y").to_string())
                        .unwrap_or_else(|| "-".into()),
                ),
            ]
            .spacing(8),
            row![stat_card("K/D", format!("{:.2}", self.kd)),].spacing(8),
        ]
        .spacing(4)
    }

    pub fn frags(&mut self, frags: Vec<Frag>) {
        self.frags.extend(frags);
        tracing::debug!("extended statistics frags");
        self.recalculate();
    }

    fn recalculate(&mut self) {
        tracing::debug!("recalculating statistics");

        self.kills = 0;
        self.deaths = 0;

        let mut unique = HashSet::new();

        for frag in &self.frags {
            if frag.is_kill() {
                self.kills += 1;
            } else {
                self.deaths += 1;
            }

            match &frag.killer {
                Target::Player(name) => {
                    unique.insert(name.clone());
                }
                _ => {}
            }

            match &frag.victim {
                Target::Player(name) => {
                    unique.insert(name.clone());
                }
                _ => {}
            }

            self.first_frag = Some(
                self.first_frag
                    .unwrap_or(frag.timestamp)
                    .min(frag.timestamp),
            );
            self.last_frag = Some(frag.timestamp.max(self.last_frag.unwrap_or(frag.timestamp)));
        }

        self.unique_players = unique.len();

        self.kd = if self.deaths == 0 {
            self.kills as f64
        } else {
            self.kills as f64 / self.deaths as f64
        };

        self.total_frags = self.frags.len();
    }
}
