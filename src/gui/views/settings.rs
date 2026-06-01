use crate::gui::element::Element;

pub struct SettingsState {}

#[derive(Debug, Clone)]
pub enum SettingsMessage {}

impl SettingsState {
    pub fn new() -> SettingsState {
        SettingsState {}
    }

    pub fn view(&self) -> Element<'_, SettingsMessage> {
        "Settings".into()
    }
}
