mod frags;
mod gui;
mod journals;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gui::main()?;

    Ok(())
}
