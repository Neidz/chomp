use std::f32;

use chrono::{Datelike, NaiveDate};
use iced::{
    event::Status,
    mouse::{Cursor, Interaction},
    widget::canvas::{self, path::Builder, Cache, Event, Frame, Geometry, LineCap, Stroke, Text},
    Color, Pixels, Point, Rectangle, Renderer, Theme,
};

use crate::app::Message;

#[derive(Debug)]
pub struct LineChart {
    cache: Cache,
    data: Vec<(NaiveDate, f32)>,
    margin: f32,
    grid_x_density: usize,
    grid_y_density: usize,
    grid_width: f32,
    axis_line_width: f32,
    text_size: f32,
    label_offset: f32,
}

#[allow(unused)]
impl LineChart {
    pub fn new(data: Vec<(NaiveDate, f32)>) -> Self {
        let mut sorted_data = data;
        sorted_data.sort_by(|(date_a, _), (date_b, _)| date_a.cmp(date_b));

        LineChart {
            cache: Cache::new(),
            data: sorted_data,
            margin: 50.0,
            grid_x_density: 5,
            grid_y_density: 20,
            grid_width: 1.0,
            axis_line_width: 2.0,
            text_size: 14.0,
            label_offset: 15.0,
        }
    }

    fn draw_data_points(&self, frame: &mut Frame, plot_area: Rectangle, stroke: Stroke<'_>) {
        if self.data.is_empty() {
            return;
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
            return;
        }

        let first_day = first_day as f32;
        let last_day = last_day as f32;

        let mut amount_of_days = last_day - first_day;
        if amount_of_days.abs() < f32::EPSILON {
            tracing::warn!("First and last day in line chart are the same");
            amount_of_days = 1.0;
        }

        let points = self
            .data
            .iter()
            .enumerate()
            .map(|(i, &(date, val))| {
                let day = date.num_days_from_ce() as f32;
                let days_since_first = day - first_day;
                let val_from_min = val - min_val;

                let x = plot_area.x + (days_since_first / amount_of_days) * plot_area.width;
                let y =
                    plot_area.y + plot_area.height - (val_from_min / val_diff) * plot_area.height;
                Point::new(x, y)
            })
            .collect::<Vec<Point>>();

        let mut builder = Builder::new();
        if let Some((first, rest)) = points.split_first() {
            builder.move_to(*first);
            for point in rest {
                builder.line_to(*point);
            }
        }

        let path = builder.build();
        frame.stroke(&path, stroke.with_width(4.0).with_line_cap(LineCap::Round));
    }

    fn draw_x_axis(
        &self,
        frame: &mut Frame,
        plot_area: Rectangle,
        stroke: Stroke<'_>,
        text_color: Color,
    ) {
        let mut builder = Builder::new();

        builder.move_to(Point::new(plot_area.x, plot_area.y + plot_area.height));
        builder.line_to(Point::new(
            plot_area.x + plot_area.width,
            plot_area.y + plot_area.height,
        ));

        let path = builder.build();
        frame.stroke(&path, stroke.with_width(self.axis_line_width));

        if self.data.is_empty() {
            return;
        }

        let first_day = self.data.first().unwrap().0.num_days_from_ce() as f32;
        let last_day = self.data.last().unwrap().0.num_days_from_ce() as f32;
        let amount_of_days = (last_day - first_day).max(1.0);

        let x_spacing = plot_area.width / self.grid_x_density as f32;

        for i in 0..=self.grid_x_density {
            let ratio = i as f32 / self.grid_x_density as f32;
            let days_offset = first_day + ratio * amount_of_days;
            let date = NaiveDate::from_num_days_from_ce_opt(days_offset as i32).unwrap();

            let label = format!("{}", date.format("%Y-%m-%d"));
            let font_size = self.text_size;
            let letter_width = font_size / 2.0;
            let text_width = label.len() as f32 * letter_width;

            let x = plot_area.x + ratio * plot_area.width - (text_width / 2.0);
            let y = plot_area.y + plot_area.height + self.label_offset;

            let text = Text {
                content: label,
                position: Point::new(x, y),
                color: text_color,
                size: Pixels::from(self.text_size),
                ..Default::default()
            };

            frame.fill_text(text);
        }
    }

    fn draw_y_axis(
        &self,
        frame: &mut Frame,
        plot_area: Rectangle,
        stroke: Stroke<'_>,
        text_color: Color,
    ) {
        let mut builder = Builder::new();

        builder.move_to(Point::new(plot_area.x, plot_area.y));
        builder.line_to(Point::new(plot_area.x, plot_area.y + plot_area.height));

        let path = builder.build();
        frame.stroke(&path, stroke.with_width(self.axis_line_width));

        if self.data.is_empty() {
            return;
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

        let y_spacing = plot_area.height / self.grid_x_density as f32;

        for i in 0..=self.grid_y_density {
            let ratio = i as f32 / self.grid_y_density as f32;
            let val = min_val + ratio * val_diff;

            let label = format!("{:.1}", val);
            let font_size = self.text_size;
            let letter_width = font_size / 2.0;
            let text_width = label.len() as f32 * letter_width;

            let x = plot_area.x - text_width - self.label_offset;
            let y = plot_area.y + plot_area.height - ratio * plot_area.height - font_size / 2.0;

            let text = Text {
                content: label,
                position: Point::new(x, y),
                color: text_color,
                size: Pixels::from(self.text_size),
                ..Default::default()
            };

            frame.fill_text(text);
        }
    }

    fn draw_grid(&self, frame: &mut Frame, plot_area: Rectangle, stroke: Stroke<'_>) {
        let mut builder = Builder::new();

        builder.move_to(Point::new(plot_area.x + plot_area.width, plot_area.y));
        builder.line_to(Point::new(
            plot_area.x + plot_area.width,
            plot_area.y + plot_area.height,
        ));

        let y_spacing = plot_area.height / self.grid_y_density as f32;
        for i in 0..self.grid_y_density {
            let x_start = plot_area.x;
            let x_end = plot_area.x + plot_area.width;
            let y = plot_area.y + i as f32 * y_spacing;
            builder.move_to(Point::new(x_start, y));
            builder.line_to(Point::new(x_end, y));
        }

        let x_spacing = plot_area.width / self.grid_x_density as f32;
        for i in 0..self.grid_x_density {
            let y_start = plot_area.y;
            let y_end = plot_area.y + plot_area.height;
            let x = plot_area.x + i as f32 * x_spacing;
            builder.move_to(Point::new(x, y_start));
            builder.line_to(Point::new(x, y_end));
        }

        let path = builder.build();
        frame.stroke(&path, stroke.with_width(self.grid_width));
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

        let plot_area = Rectangle {
            x: self.margin,
            y: self.margin,
            width: bounds.width - 2.0 * self.margin,
            height: bounds.height - 2.0 * self.margin,
        };

        let text_color = theme.extended_palette().primary.base.color;
        let data_line_stroke = Stroke::default().with_color(text_color);
        let grid_stroke = Stroke::default();

        let graph = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_grid(frame, plot_area, grid_stroke);
            self.draw_x_axis(frame, plot_area, grid_stroke, text_color);
            self.draw_y_axis(frame, plot_area, grid_stroke, text_color);
            self.draw_data_points(frame, plot_area, data_line_stroke);
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
