use {
    crate::{frags::Frag, gui::element::Element},
    chrono::{DateTime, NaiveDate, Utc},
    iced::Color,
    iced_plot::{PlotUiMessage, PlotWidget, PlotWidgetBuilder, Series},
    std::collections::BTreeMap,
};

pub struct PlotState {
    widget: PlotWidget,
    kills: BTreeMap<NaiveDate, i32>,
    deaths: BTreeMap<NaiveDate, i32>,
}

#[derive(Debug, Clone)]
pub enum PlotMessage {
    Widget(PlotUiMessage),
}

impl PlotState {
    pub fn new() -> PlotState {
        PlotState {
            widget: PlotWidget::default(),
            kills: BTreeMap::new(),
            deaths: BTreeMap::new(),
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
            let date = frag.timestamp.date_naive();

            if frag.is_kill() {
                *self.kills.entry(date).or_default() += 1;
            } else {
                *self.deaths.entry(date).or_default() += 1;
            }
        }

        tracing::debug!("rebuilding plot");
        tracing::debug!(kills = self.kills.len(), deaths = self.deaths.len());
        self.rebuild_plot();
    }

    fn rebuild_plot(&mut self) {
        if self.kills.len() < 2 && self.deaths.len() < 2 {
            return;
        }

        let kill_points: Vec<[f64; 2]> = self
            .kills
            .iter()
            .map(|(day, count)| {
                let x = day.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as f64;

                [x, *count as f64]
            })
            .collect();

        let death_points: Vec<[f64; 2]> = self
            .deaths
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
