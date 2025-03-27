use chrono::NaiveDate;
use iced::{
    widget::{column, row, Canvas, Text},
    Element,
    Length::{self},
};

use crate::app::{Context, Message};

use super::{sidebar::sidebar, LineChart, Widget};

#[derive(Debug, Clone)]
pub enum DashboardMessage {}

impl From<DashboardMessage> for Message {
    fn from(value: DashboardMessage) -> Self {
        Message::Dashboard(value)
    }
}

#[derive(Debug)]
pub struct Dashboard {
    chart: LineChart,
}

impl Dashboard {
    pub fn new() -> Self {
        let test_data: Vec<(NaiveDate, f32)> = vec![
            (NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), 82.5),
            (NaiveDate::from_ymd_opt(2025, 1, 5).unwrap(), 81.9),
            (NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(), 81.4),
            (NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(), 80.8),
            (NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(), 80.3),
            (NaiveDate::from_ymd_opt(2025, 1, 25).unwrap(), 79.7),
            (NaiveDate::from_ymd_opt(2025, 1, 30).unwrap(), 79.2),
        ];

        Dashboard {
            chart: LineChart::new(test_data),
        }
    }
}

impl Widget for Dashboard {
    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(&self.chart)
            .width(Length::Fill)
            .height(Length::Fill);
        let content = column![Text::new("Dashboard").size(40), canvas].spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, _ctx: &mut Context, _msg: Message) {}
}
