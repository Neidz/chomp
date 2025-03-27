use std::f32;

use chrono::{Datelike, NaiveDate};
use iced::{
    event::Status,
    mouse::{Cursor, Interaction},
    widget::canvas::{self, path::Builder, Cache, Event, Geometry, Stroke},
    Point, Rectangle, Renderer, Theme,
};

use crate::app::Message;

#[derive(Debug)]
pub struct LineChart {
    data: Vec<(NaiveDate, f32)>,
    cache: Cache,
}

#[allow(unused)]
impl LineChart {
    pub fn new(data: Vec<(NaiveDate, f32)>) -> Self {
        let mut sorted_data = data;
        sorted_data.sort_by(|(date_a, _), (date_b, _)| date_a.cmp(date_b));

        LineChart {
            data: sorted_data,
            cache: Cache::new(),
        }
    }

    fn points(&self, frame_width: f32, frame_height: f32) -> Vec<Point> {
        if self.data.is_empty() {
            return vec![];
        }

        let mut min_val = self.data[0].1;
        let mut max_val = self.data[0].1;

        for &(_, val) in self.data.iter() {
            if min_val > val {
                min_val = val;
            }
            if max_val < val {
                max_val = val;
            }
        }

        let mut val_diff = max_val - min_val;
        if val_diff.abs() < f32::EPSILON {
            tracing::warn!("First and last value in line chart are the same");
            val_diff = 1.0;
        }

        let first_day = self.data.first().unwrap().0.num_days_from_ce();
        let last_day = self.data.last().unwrap().0.num_days_from_ce();

        if first_day < 0 {
            tracing::error!("First day in line chart is before year 1970");
            panic!();
        }

        let first_day = first_day as f32;
        let last_day = last_day as f32;

        let mut amount_of_days = last_day - first_day;
        if amount_of_days.abs() < f32::EPSILON {
            tracing::warn!("First and last day in line chart are the same");
            amount_of_days = 1.0;
        }

        self.data
            .iter()
            .enumerate()
            .map(|(i, &(date, val))| {
                let day = date.num_days_from_ce() as f32;
                let days_since_first = day - first_day;
                let val_from_min = val - min_val;

                let x = 0.0 + (days_since_first / amount_of_days) * frame_width;
                let y = frame_height - (val_from_min / val_diff) * frame_height;
                Point::new(x, y)
            })
            .collect()
    }
}

impl canvas::Program<Message> for LineChart {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<Message>) {
        (Status::Captured, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        if self.data.is_empty() {
            return vec![];
        }

        let text_color = theme.extended_palette().primary.base.color;
        let graph_fill = canvas::Fill {
            style: canvas::Style::Solid(text_color),
            ..Default::default()
        };
        let stroke = Stroke::default().with_width(4.0);

        let graph = self.cache.draw(renderer, bounds.size(), |frame| {
            let width = frame.width();
            let height = frame.height();

            let mut builder = Builder::new();
            let points = self.points(width, height);
            if let Some((first, rest)) = points.split_first() {
                builder.move_to(*first);
                for point in rest {
                    builder.line_to(*point);
                }
            }

            let path = builder.build();
            frame.stroke(&path, stroke);
        });
        vec![graph]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::None
    }
}
