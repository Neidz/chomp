use iced::{
    widget::{column, row, Text},
    Element,
    Length::{self},
};

use crate::app::{Context, Message};

use super::{sidebar::sidebar, Widget};

#[derive(Debug, Clone)]
pub enum DashboardMessage {}

impl From<DashboardMessage> for Message {
    fn from(value: DashboardMessage) -> Self {
        Message::Dashboard(value)
    }
}

#[derive(Debug)]
pub struct Dashboard {}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {}
    }
}

impl Widget for Dashboard {
    fn view(&self) -> Element<Message> {
        let content = column![Text::new("Dashboard").size(40)].spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, _ctx: &mut Context, _msg: Message) {}
}
