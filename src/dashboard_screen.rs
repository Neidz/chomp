use iced::{
    widget::{column, row, Text},
    Element, Length,
};

use crate::{
    app::{App, Message},
    sidebar::render_sidebar,
};

pub fn render_dashboard_screen(app: &App) -> Element<Message> {
    let content = column![Text::new("Home").size(40)].spacing(10);

    row![render_sidebar(app), content]
        .height(Length::Fill)
        .padding(20)
        .spacing(20)
        .into()
}
