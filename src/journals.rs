use {
    crate::{frags::Frag, ship::Ship},
    edjr::{Journal, JournalEvent},
    futures::{Stream, StreamExt, stream},
    std::path::{Path, PathBuf},
    tokio::{
        fs::{File, read_dir},
        io::Error,
    },
};

pub async fn get_journals(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
    let mut dir = read_dir(path).await?;
    let mut paths = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "log") {
            paths.push(path);
        }
    }

    Ok(paths)
}

pub enum ScanError {
    Journal(edjr::error::JournalError),
}

pub async fn scan_journal(path: impl AsRef<Path>) -> Result<Vec<Frag>, ScanError> {
    let journal = Journal::<File>::open(path)
        .await
        .map_err(ScanError::Journal)?;
    let mut stream = journal.stream().boxed();

    let mut ship: Option<Ship> = None;
    let mut star_system: Option<String> = None;
    let mut frags = Vec::new();

    while let Some(Ok(entry)) = stream.next().await {
        match entry.event {
            JournalEvent::Loadout(event) => {
                ship = Some(Ship::from(event.ship));
            }
            JournalEvent::Location(event) => {
                star_system = Some(event.star_system);
            }
            JournalEvent::FsdJump(event) => {
                star_system = Some(event.star_system);
            }
            JournalEvent::PvpKill(event) => {
                frags.push(Frag::kill(
                    entry.timestamp,
                    star_system.clone(),
                    ship.clone(),
                    event.victim,
                ));
            }
            JournalEvent::Died(event) => {
                if let Some(killer_name) = event.killer_name {
                    if !killer_name.contains("Cmdr") {
                        continue;
                    }

                    frags.push(Frag::death(
                        entry.timestamp,
                        star_system.clone(),
                        ship.clone(),
                        strip_cmdr(&killer_name).to_string(),
                    ));
                } else if let Some(killers) = event.killers {
                    let first = killers.into_iter().next();

                    if let Some(killer) = first {
                        frags.push(Frag::death(
                            entry.timestamp,
                            star_system.clone(),
                            ship.clone(),
                            strip_cmdr(&killer.name).to_string(),
                        ));
                    }
                }
            }
            _ => (),
        }
    }

    Ok(frags)
}

pub fn scan_journals(journals: Vec<PathBuf>) -> impl Stream<Item = Result<Vec<Frag>, ScanError>> {
    stream::iter(journals).map(scan_journal).buffer_unordered(8)
}

fn strip_cmdr(content: &str) -> &str {
    content.strip_prefix("Cmdr ").unwrap_or(&content)
}
