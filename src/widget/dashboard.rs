use iced::{
    widget::{column, row, Canvas, Text},
    Element,
    Length::{self},
};

use crate::{
    app::{Context, Message},
    data::Weight,
};

use super::{line_chart::LineChartEntry, sidebar::sidebar, LineChart, Widget};

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
    pub fn new(weights: Vec<Weight>) -> Self {
        let weights: Vec<LineChartEntry> = weights.into_iter().map(|w| (w.day, w.weight)).collect();

        Dashboard {
            chart: LineChart::new(weights),
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
