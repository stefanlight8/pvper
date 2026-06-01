use iced::Result;

use crate::gui::state::State;

pub fn main() -> Result {
    iced::application(State::new, State::update, State::view).run()
}
