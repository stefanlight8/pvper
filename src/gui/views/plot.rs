use crate::gui::element::Element;

use chrono::{DateTime, Utc};
use iced_plot::{PlotUiMessage, PlotWidget, PlotWidgetBuilder};

pub struct Point {
    tick: DateTime<Utc>,
}

pub struct PlotState {
    widget: PlotWidget,
}

#[derive(Debug, Clone)]
pub enum PlotMessage {
    Widget(PlotUiMessage),
}

impl PlotState {
    pub fn new() -> PlotState {
        PlotState {
            widget: PlotWidgetBuilder::new().build().unwrap(),
        }
    }

    pub fn update(&mut self, message: PlotMessage) {
        match message {
            PlotMessage::Widget(message) => self.widget.update(message),
        }
    }

    pub fn view(&self) -> Element<'_, PlotMessage> {
        self.widget.view().map(PlotMessage::Widget).into()
    }
}
