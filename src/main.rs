#![windows_subsystem = "windows"]

mod dialogs;
mod frags;
mod fs;
mod gui;
mod journals;
mod settings;
mod ship;

use std::error::Error;

use dirs::config_dir;
use tracing_subscriber::EnvFilter;


const SETTINGS_FILE: &str = "settings.json";

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config_dir = config_dir().unwrap().join("pvped");

    gui::main(config_dir)?;

    Ok(())
}
