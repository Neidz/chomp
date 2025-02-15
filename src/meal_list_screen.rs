use chrono::NaiveDate;
use iced::{
    widget::{column, progress_bar, row, vertical_space, Button, Container, Scrollable, Text},
    Alignment::Center,
    Element, Length,
};

use crate::{
    app::{App, Message},
    data::{Meal, MealDayStats, MealProduct},
    meal_product_form::{
        render_add_product_to_meal_form, render_copy_meal_products_form,
        render_update_meal_product_form,
    },
    modal::render_modal,
    sidebar::render_sidebar,
    style::TableRowStyle,
};

#[derive(Debug, Clone)]
pub enum MealListMessage {
    CreateMealProductFormMeal(Option<usize>),
    CreateMealProductFormWeight(String),
    CreateMealProductFormProduct(usize),
    SubmitAddMealProductForm,

    UpdateMealProductFormMealProduct(Option<usize>),
    UpdateMealProductFormWeight(String),
    SubmitUpdateMealProductForm,

    DeleteMealProduct(usize),

    CopyMealProductsMeal(Option<usize>),
    CopyMealProductsFromDay(NaiveDate),
    SubmitCopyMealProductsForm,
}

impl From<MealListMessage> for Message {
    fn from(value: MealListMessage) -> Self {
        Message::MealList(value)
    }
}

pub fn render_meal_list_screen(app: &App) -> Element<Message> {
    let mut tables = column![].spacing(20);
    for meal in app.meals.iter() {
        tables = tables.push(render_meal(meal))
    }

    let content = column![
        Text::new("Meals").size(40),
        Scrollable::new(tables),
        vertical_space(),
        meal_stats(&app.meal_day_stats)
    ]
    .spacing(10);

    let content_with_sidebar = row![render_sidebar(app), content]
        .height(Length::Fill)
        .padding(20)
        .spacing(20);

    if let Some(add_form) = &app.add_meal_product_form {
        return render_modal(
            content_with_sidebar.into(),
            render_add_product_to_meal_form(add_form),
            MealListMessage::CreateMealProductFormMeal(None).into(),
        );
    }

    if let Some(update_form) = &app.update_meal_product_form {
        return render_modal(
            content_with_sidebar.into(),
            render_update_meal_product_form(update_form),
            MealListMessage::UpdateMealProductFormMealProduct(None).into(),
        );
    }

    if let Some(copy_form) = &app.copy_meal_products_form {
        return render_modal(
            content_with_sidebar.into(),
            render_copy_meal_products_form(copy_form),
            MealListMessage::CopyMealProductsMeal(None).into(),
        );
    }

    content_with_sidebar.into()
}

fn render_meal(meal: &Meal) -> Element<Message> {
    let mut table = column![
        row![
            Text::new(&meal.name).size(20),
            Button::new("Add Product")
                .on_press(MealListMessage::CreateMealProductFormMeal(Some(meal.id)).into()),
            Button::new("Copy From Different Day")
                .on_press(MealListMessage::CopyMealProductsMeal(Some(meal.id)).into())
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
            Button::new("Update")
                .on_press(MealListMessage::UpdateMealProductFormMealProduct(Some(mp.id)).into()),
            Button::new("Delete").on_press(MealListMessage::DeleteMealProduct(mp.id).into())
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
