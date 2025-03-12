use iced::{
    widget::{column, Button, Column},
    Element, Length,
};

use crate::app::{Message, NextWidget};

pub fn sidebar() -> Element<'static, Message> {
    let buttons = vec![
        ("Dashboard", Message::ChangeWidget(NextWidget::Dashboard)),
        ("Meal List", Message::ChangeWidget(NextWidget::MealList)),
        (
            "Product List",
            Message::ChangeWidget(NextWidget::ProductList),
        ),
        (
            "Calorie Target List",
            Message::ChangeWidget(NextWidget::CalorieTargetList),
        ),
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
