use std::path::Path;

use async_stream::stream;
use chrono::{DateTime, Utc};
use edjr::{Journal, JournalEvent};
use futures_lite::Stream;
use iced::futures::StreamExt;
use tokio::fs::File;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    You,
    Player(String),
}

#[derive(Debug, Clone)]
pub struct Frag {
    timestamp: DateTime<Utc>,
    killer: Target,
    victim: Target,
}

impl Frag {
    pub fn is_kill(&self) -> bool {
        self.victim != Target::You
    }

    pub fn death(timestamp: DateTime<Utc>, player: String) -> Frag {
        Frag {
            timestamp,
            killer: Target::Player(player),
            victim: Target::You,
        }
    }

    pub fn kill(timestamp: DateTime<Utc>, player: String) -> Frag {
        Frag {
            timestamp,
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

        while let Some(Ok(entry)) = stream.next().await {
            match entry.event {
                JournalEvent::PvpKill(event) => {
                    yield Ok(Frag::kill(entry.timestamp, event.victim));
                }
                JournalEvent::Died(event) => {
                    if let Some(killer_name) = event.killer_name {
                        yield Ok(Frag::death(entry.timestamp, killer_name));
                    } else if let Some(killers) = event.killers {
                        let first = killers.into_iter().next();

                        if let Some(killer) = first {
                            yield Ok(Frag::death(entry.timestamp, killer.name));
                        }
                    }
                }
                _ => ()
            }
        }
    }
}
