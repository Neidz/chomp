use iced::{
    widget::{column, Button, Column},
    Element, Length,
};

use crate::app::{Message, NextWidget};

pub fn sidebar() -> Element<'static, Message> {
    let buttons = vec![
        ("Dashboard", Message::ChangeWidget(NextWidget::Dashboard)),
        ("Meals", Message::ChangeWidget(NextWidget::MealList)),
        ("Products", Message::ChangeWidget(NextWidget::ProductList)),
        ("Weights", Message::ChangeWidget(NextWidget::WeightList)),
        (
            "Nutrition Targets",
            Message::ChangeWidget(NextWidget::NutritionTargetList),
        ),
        ("Tools", Message::ChangeWidget(NextWidget::Tools)),
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
