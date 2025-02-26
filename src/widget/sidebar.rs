use iced::{
    widget::{column, Button, Column},
    Element, Length,
};

use crate::app::{Message, NextWidget};

pub fn sidebar() -> Element<'static, Message> {
    let buttons = vec![
        ("Dashboard", Message::ChangeWidget(NextWidget::Dashboard)),
        (
            "Create Product",
            Message::ChangeWidget(NextWidget::CreateProduct),
        ),
        ("Meal List", Message::ChangeWidget(NextWidget::MealList)),
    ];

    let navigation = buttons
        .into_iter()
        .map(|(content, message)| {
            Button::new(content)
                .on_press(message)
                .width(Length::Fill)
                .into()
        })
        .collect::<Column<Message>>()
        .width(200)
        .spacing(10);

    column![navigation].into()
}
