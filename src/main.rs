mod frags;
mod gui;
mod journals;
mod ship;

use std::error::Error;

use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    gui::main()?;

    Ok(())
}
