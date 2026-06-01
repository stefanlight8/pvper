use iced::Result;

use crate::gui::state::State;

pub fn main() -> Result {
    iced::application(State::new, State::update, State::view)
        .theme(State::theme)
        .title("Pvper")
        .run()
}
