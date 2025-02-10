use iced::{
    widget::{column, horizontal_space, row, Button, Container, Scrollable, Text},
    Element, Length,
};

use crate::{
    app::Message,
    data::{Meal, MealProduct},
    style::TableRowStyle,
};

pub fn render_meal_list(meals: &[Meal]) -> Element<Message> {
    let mut tables = column![].spacing(10);
    for (_, meal) in meals.iter().enumerate() {
        tables = tables.push(render_meal(meal))
    }

    Scrollable::new(tables).into()
}

fn render_meal(meal: &Meal) -> Element<Message> {
    let mut table = column![
        row![
            Text::new(&meal.name).size(20),
            horizontal_space(),
            Button::new("Add Product")
                .on_press(Message::UpdateAddMealProductFormMeal(Some(meal.id)))
        ],
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
        Text::new(format!("{:.2}", mp.weight)).width(Length::Fill),
        Text::new(format!("{:.2}", mp.calories)).width(Length::Fill),
        Text::new(format!("{:.2}", mp.fats)).width(Length::Fill),
        Text::new(format!("{:.2}", mp.proteins)).width(Length::Fill),
        Text::new(format!("{:.2}", mp.carbohydrates)).width(Length::Fill),
        row![].spacing(10).width(Length::Fill)
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
