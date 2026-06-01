use std::path::PathBuf;

pub fn user_home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}
