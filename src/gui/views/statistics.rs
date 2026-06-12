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
    total_frags: usize,
    kills: usize,
    deaths: usize,
    kd: f64,
    unique_players: HashSet<String>,
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
                stat_card("Players", self.unique_players.len().to_string()),
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
                        .unwrap_or_else(|| "-".to_string()),
                ),
                stat_card(
                    "Last fight",
                    self.last_frag
                        .map(|t| t.format("%d %b %Y").to_string())
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ]
            .spacing(8),
            row![stat_card("K/D", format!("{:.2}", self.kd))].spacing(8),
        ]
        .spacing(4)
    }

    pub fn update(&mut self, frags: &[Frag]) {
        for frag in frags {
            self.kills += frag.is_kill() as usize;
            self.deaths += !frag.is_kill() as usize;
            self.kd = if self.deaths == 0 {
                self.kills as f64
            } else {
                self.kills as f64 / self.deaths as f64
            };
            self.total_frags += 1;

            if self.first_frag.map_or(true, |t| frag.timestamp < t) {
                self.first_frag = Some(frag.timestamp);
            }

            if self.last_frag.map_or(true, |t| frag.timestamp > t) {
                self.last_frag = Some(frag.timestamp);
            }

            if let Target::Player(name) = &frag.killer {
                self.unique_players.insert(name.clone());
            }

            if let Target::Player(name) = &frag.victim {
                self.unique_players.insert(name.clone());
            }
        }
    }
}
