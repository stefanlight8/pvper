use std::path::{Path, PathBuf};
use tokio::{fs::read_dir, io::Error};

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
