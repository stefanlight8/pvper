use std::path::Path;

use async_stream::stream;
use chrono::{DateTime, Utc};
use edjr::{Journal, JournalEvent};
use futures_lite::Stream;
use iced::futures::StreamExt;
use tokio::fs::File;

use crate::ship::Ship;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    You,
    Player(String),
}

#[derive(Debug, Clone)]
pub struct Frag {
    pub timestamp: DateTime<Utc>,
    pub killer: Target,
    pub victim: Target,
    pub star_system: Option<String>,
    pub ship: Option<Ship>,
}

impl Frag {
    pub fn is_kill(&self) -> bool {
        self.victim != Target::You
    }

    pub fn death(
        timestamp: DateTime<Utc>,
        star_system: Option<String>,
        ship: Option<Ship>,
        player: String,
    ) -> Frag {
        Frag {
            timestamp,
            star_system,
            ship,
            killer: Target::Player(player),
            victim: Target::You,
        }
    }

    pub fn kill(
        timestamp: DateTime<Utc>,
        star_system: Option<String>,
        ship: Option<Ship>,
        player: String,
    ) -> Frag {
        Frag {
            timestamp,
            star_system,
            ship,
            killer: Target::You,
            victim: Target::Player(player),
        }
    }
}

pub enum ScanError {
    Journal(edjr::error::JournalError),
}

pub fn scan_journal(path: impl AsRef<Path>) -> impl Stream<Item = Result<Frag, ScanError>> {
    stream! {
        let journal = Journal::<File>::open(path)
            .await
            .map_err(ScanError::Journal)?;
        let mut stream = journal.stream().boxed();

        let mut ship: Option<Ship> = None;
        let mut star_system: Option<String> = None;

        while let Some(Ok(entry)) = stream.next().await {
            match entry.event {
                JournalEvent::Loadout(event) => {
                    ship = Some(Ship::from(event.ship));
                }
                JournalEvent::Location(event) => {
                    star_system = Some(event.star_system);
                }
                JournalEvent::PvpKill(event) => {
                    yield Ok(Frag::kill(entry.timestamp, star_system.clone(), ship.clone(), event.victim));
                }
                JournalEvent::Died(event) => {
                    if let Some(killer_name) = event.killer_name {
                        if !killer_name.contains("Cmdr") {
                            continue
                        }

                        yield Ok(Frag::death(entry.timestamp, star_system.clone(), ship.clone(), strip_cmdr(&killer_name).to_string()));
                    } else if let Some(killers) = event.killers {
                        let first = killers.into_iter().next();

                        if let Some(killer) = first {
                            yield Ok(Frag::death(entry.timestamp, star_system.clone(), ship.clone(), strip_cmdr(&killer.name).to_string()));
                        }
                    }
                }
                _ => ()
            }
        }
    }
}

fn strip_cmdr(content: &str) -> &str {
    content.strip_prefix("Cmdr ").unwrap_or(&content)
}
