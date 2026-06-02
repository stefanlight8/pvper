use {
    crate::fs::user_home,
    serde::{Deserialize, Serialize},
    std::path::PathBuf,
    tokio::{
        fs::{create_dir_all, read, write},
        io,
    },
};

pub enum SettingsError {
    Io(io::Error),
    Parse(serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct Settings {
    path: PathBuf,
    settings: SettingsData,
}

impl Settings {
    pub fn new(path: PathBuf, settings: SettingsData) -> Settings {
        Settings { path, settings }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsData {
    pub journals_path: PathBuf,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            journals_path: user_home()
                .join("Saved Games")
                .join("Frontier Developments")
                .join("Elite Dangerous"),
        }
    }
}

impl Settings {
    pub async fn load(path: PathBuf) -> Result<Settings, SettingsError> {
        let settings_data: SettingsData;
        let settings: Settings;

        if !path.exists() {
            settings_data = SettingsData::default();
            settings = Settings::new(path, settings_data);
            settings.save().await?;
        } else {
            let data = read(&path).await.map_err(SettingsError::Io)?;
            settings_data = serde_json::from_slice(&data).map_err(SettingsError::Parse)?;
            settings = Settings::new(path, settings_data);
        };

        Ok(settings)
    }

    pub async fn save(&self) -> Result<(), SettingsError> {
        let data = serde_json::to_string_pretty(&self.settings).map_err(SettingsError::Parse)?;

        if !self.path.exists() {
            create_dir_all(self.path.parent().unwrap())
                .await
                .map_err(SettingsError::Io)?;
        }

        write(&self.path, data).await.map_err(SettingsError::Io)
    }

    pub fn journals_path(&self) -> PathBuf {
        self.settings.journals_path.clone()
    }

    pub fn set_journals_path(&mut self, path: PathBuf) {
        self.settings.journals_path = path.to_owned();
    }
}
