use chomp_services::Weight;
use iced::{
    widget::{column, row, Canvas, Text},
    Element,
    Length::{self},
    Task,
};

use crate::{
    app::{Context, Message},
    widget::{modal::modal, DatePicker},
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
    date_picker: DatePicker,
}

impl Dashboard {
    pub fn new(weights: Vec<Weight>) -> Self {
        let weights: Vec<LineChartEntry> = weights.into_iter().map(|w| (w.day, w.weight)).collect();

        Dashboard {
            chart: LineChart::new(weights),
            date_picker: DatePicker::new("Test date"),
        }
    }
}

impl Widget for Dashboard {
    fn view(&self) -> Element<Message> {
        let date_picker = self
            .date_picker
            .view(|new_date| Message::DatePickerDateChange(new_date));
        let canvas = Canvas::new(&self.chart)
            .width(Length::Fill)
            .height(Length::Fill);
        let content = column![Text::new("Dashboard").size(40), date_picker, canvas].spacing(10);

        let content_with_sidebar = row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20);

        if self.date_picker.calendar_open() {
            return modal(
                content_with_sidebar.into(),
                self.date_picker
                    .view_modal(|new_date| Message::DatePickerDateChange(new_date)),
                Message::CloseDatePicker.into(),
            );
        }

        content_with_sidebar.into()
    }

    fn update(&mut self, _ctx: &mut Context, msg: Message) -> Task<Message> {
        self.date_picker.handle_message(msg);

        Task::none()
    }
}
