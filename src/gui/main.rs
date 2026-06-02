use {crate::gui::state::State, iced::Result, std::path::PathBuf};

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
