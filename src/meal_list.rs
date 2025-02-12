use iced::{
    widget::{column, progress_bar, row, Button, Container, Scrollable, Text},
    Alignment::Center,
    Element, Length,
};

use crate::{
    app::Message,
    data::{Meal, MealDayStats, MealProduct},
    style::TableRowStyle,
};

pub fn render_meal_list(meals: &[Meal]) -> Element<Message> {
    let mut tables = column![].spacing(10);
    for meal in meals.iter() {
        tables = tables.push(render_meal(meal))
    }

    Scrollable::new(tables).into()
}

fn render_meal(meal: &Meal) -> Element<Message> {
    let mut table = column![
        row![
            Text::new(&meal.name).size(20),
            Button::new("Add Product").on_press(Message::CreateMealProductFormMeal(Some(meal.id)))
        ]
        .spacing(10),
        list_header_row()
    ];

    for (i, meal_product) in meal.products.iter().enumerate() {
        table = table.push(list_row(meal_product, i % 2 == 0));
    }

    table.into()
}

fn list_header_row() -> Element<'static, Message> {
    let row = row![
        Text::new("Name").width(Length::Fill),
        Text::new("Weight (g)").width(Length::Fill),
        Text::new("Calories (kcal)").width(Length::Fill),
        Text::new("Fats (g)").width(Length::Fill),
        Text::new("Proteins (g)").width(Length::Fill),
        Text::new("Carbohydrates (g)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row(mp: &MealProduct, even: bool) -> Element<Message> {
    let row = row![
        Text::new(&mp.name).width(Length::Fill),
        Text::new(format!("{:.1}", mp.weight)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.calories)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.fats)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.proteins)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.carbohydrates)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::UpdateMealProductFormMealProduct(Some(mp.id))),
            Button::new("Delete").on_press(Message::DeleteMealProduct(mp.id))
        ]
        .spacing(10)
        .width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row)
        .width(Length::Fill)
        .style(move |t| {
            if even {
                TableRowStyle::Even.style(t)
            } else {
                TableRowStyle::Odd.style(t)
            }
        })
        .into()
}

pub fn meal_stats(stats: &MealDayStats) -> Element<Message> {
    row![
        meal_stat("Calories", stats.calories, 2500.0),
        meal_stat("Proteins", stats.proteins, 200.0),
        meal_stat("Fats", stats.fats, 60.0),
        meal_stat("Carbohydrates", stats.carbohydrates, 300.0)
    ]
    .spacing(40)
    .width(Length::Fill)
    .into()
}

fn meal_stat(label: &str, value: f32, max_value: f32) -> Element<Message> {
    column![
        Text::new(format!("{} {:.1}/{:.1}", label, value, max_value)),
        progress_bar(0.0..=100.0, value / max_value * 100.0),
    ]
    .align_x(Center)
    .spacing(2)
    .into()
}
