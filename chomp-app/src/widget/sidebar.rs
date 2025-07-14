use iced::{
    widget::{column, row, svg, Button, Column},
    Element, Length,
};

use crate::app::{Message, NextWidget};

pub fn sidebar() -> Element<'static, Message> {
    let dashboard_icon = svg::Handle::from_path(format!(
        "{}/resources/chart-pie.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let product_icon =
        svg::Handle::from_path(format!("{}/resources/beef.svg", env!("CARGO_MANIFEST_DIR")));
    let meal_icon = svg::Handle::from_path(format!(
        "{}/resources/salad.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let weight_icon = svg::Handle::from_path(format!(
        "{}/resources/weight.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let target_icon = svg::Handle::from_path(format!(
        "{}/resources/target.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let tool_icon = svg::Handle::from_path(format!(
        "{}/resources/wrench.svg",
        env!("CARGO_MANIFEST_DIR")
    ));

    let buttons = vec![
        (
            dashboard_icon.clone(),
            "Dashboard",
            Message::ChangeWidget(NextWidget::Dashboard),
        ),
        (
            meal_icon.clone(),
            "Meals",
            Message::ChangeWidget(NextWidget::MealList),
        ),
        (
            product_icon.clone(),
            "Products",
            Message::ChangeWidget(NextWidget::ProductList),
        ),
        (
            weight_icon.clone(),
            "Weights",
            Message::ChangeWidget(NextWidget::WeightList),
        ),
        (
            target_icon.clone(),
            "Nutrition Targets",
            Message::ChangeWidget(NextWidget::NutritionTargetList),
        ),
        (
            tool_icon.clone(),
            "Tools",
            Message::ChangeWidget(NextWidget::Tools),
        ),
    ];

    let navigation = buttons
        .into_iter()
        .map(|(icon, content, message)| {
            Button::new(row![svg(icon).width(20.0), content].spacing(10.0))
                .on_press(message)
                .width(Length::Fill)
                .into()
        })
        .collect::<Column<Message>>()
        .width(200)
        .spacing(10);

    column![navigation].into()
}
