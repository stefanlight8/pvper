use std::collections::BTreeMap;

use crate::{frags::Frag, gui::element::Element};

use chrono::{DateTime, NaiveDate, Utc};
use iced::Color;
use iced_plot::{PlotUiMessage, PlotWidget, PlotWidgetBuilder, Series};

pub struct PlotState {
    widget: PlotWidget,
    kills: Vec<DateTime<Utc>>,
    deaths: Vec<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum PlotMessage {
    Widget(PlotUiMessage),
}

impl PlotState {
    pub fn new() -> PlotState {
        PlotState {
            widget: PlotWidget::default(),
            kills: Vec::new(),
            deaths: Vec::new(),
        }
    }

    pub fn update(&mut self, message: PlotMessage) {
        match message {
            PlotMessage::Widget(message) => self.widget.update(message),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, PlotMessage> {
        self.widget.view().map(PlotMessage::Widget).into()
    }

    pub fn frags(&mut self, frags: Vec<Frag>) {
        for frag in frags {
            if frag.is_kill() {
                self.kills.push(frag.timestamp);
            } else {
                self.deaths.push(frag.timestamp);
            }
        }

        self.rebuild_plot();
    }

    fn rebuild_plot(&mut self) {
        let mut kills_per_day: BTreeMap<NaiveDate, i32> = BTreeMap::new();
        let mut deaths_per_day: BTreeMap<NaiveDate, i32> = BTreeMap::new();

        for ts in &self.kills {
            *kills_per_day.entry(ts.date_naive()).or_default() += 1;
        }

        for ts in &self.deaths {
            *deaths_per_day.entry(ts.date_naive()).or_default() += 1;
        }

        let kill_points: Vec<[f64; 2]> = kills_per_day
            .iter()
            .map(|(day, count)| {
                let x = day.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as f64;

                [x, *count as f64]
            })
            .collect();

        let death_points: Vec<[f64; 2]> = deaths_per_day
            .iter()
            .map(|(day, count)| {
                let x = day.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as f64;

                [x, -(*count as f64)]
            })
            .collect();

        let kills_series = Series::line_only(kill_points, iced_plot::LineStyle::Solid)
            .with_color(Color::from_rgb(0.2, 0.6, 1.0))
            .with_label("Kills");

        let deaths_series = Series::line_only(death_points, iced_plot::LineStyle::Solid)
            .with_color(Color::from_rgb(1.0, 0.2, 0.2))
            .with_label("Deaths");

        self.widget = PlotWidgetBuilder::new()
            .add_series(kills_series)
            .add_series(deaths_series)
            .with_x_tick_formatter(|tick| {
                DateTime::<Utc>::from_timestamp(tick.value as i64, 0)
                    .map(|dt| dt.format("%d %b").to_string())
                    .unwrap_or_default()
            })
            .with_cursor_provider(|x, y| {
                let date = DateTime::<Utc>::from_timestamp(x as i64, 0)
                    .map(|dt| dt.format("%d %b, %Y").to_string())
                    .unwrap_or_else(|| "?".into());

                format!("{date} | {y:.0}")
            })
            .build()
            .unwrap_or_default();
    }
}
