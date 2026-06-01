use std::path::PathBuf;

use iced::Result;

use crate::gui::state::State;

pub fn main(config_dir: PathBuf) -> Result {
    iced::application(
        move || State::boot(config_dir.clone()),
        State::update,
        State::view,
    )
    .theme(State::theme)
    .title("pvped")
    .run()
}
