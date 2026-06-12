use {
    crate::{frags::Frag, ship::Ship},
    edjr::{Journal, JournalEvent},
    futures::{Stream, StreamExt, stream},
    std::{
        fmt::Display,
        path::{Path, PathBuf},
    },
    tokio::{
        fs::{File, read_dir},
        io::Error,
    },
    tracing::instrument,
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

    tracing::debug!("found {} journals", paths.len());

    Ok(paths)
}

#[derive(Debug)]
pub enum ScanError {
    Journal(edjr::error::JournalError),
}

impl std::error::Error for ScanError {}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::Journal(err) => write!(f, "failed to open journal: {}", err),
        }
    }
}

#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn scan_journal(path: impl AsRef<Path>) -> Result<Vec<Frag>, ScanError> {
    tracing::debug!("reading journal");

    let journal = Journal::<File>::open(&path)
        .await
        .map_err(ScanError::Journal)?;
    let mut stream = journal.stream().boxed();

    let mut ship: Option<Ship> = None;
    let mut star_system: Option<String> = None;
    let mut frags = Vec::new();

    while let Some(entry) = stream.next().await {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                tracing::error!(
                    "failed to read journal ({}): {}",
                    &path.as_ref().display(),
                    err
                );
                continue;
            }
        };

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

    tracing::debug!("found {} frags", frags.len());

    Ok(frags)
}

pub fn scan_journals(journals: Vec<PathBuf>) -> impl Stream<Item = Result<Vec<Frag>, ScanError>> {
    stream::iter(journals)
        .map(scan_journal)
        .buffer_unordered(8)
        .filter_map(|res| async move {
            match res {
                Ok(frags) if !frags.is_empty() => Some(Ok(frags)),
                Ok(_) => None,
                Err(e) => Some(Err(e)),
            }
        })
}

fn strip_cmdr(content: &str) -> &str {
    content.strip_prefix("Cmdr ").unwrap_or(&content)
}
