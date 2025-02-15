use chrono::{Days, Local};
use iced::{
    widget::{column, horizontal_space, row, vertical_space, Button, Column, Text},
    Alignment::Center,
    Element, Length,
};

use crate::app::{App, Message, Screen};

pub fn render_sidebar(app: &App) -> Element<Message> {
    let buttons = vec![
        ("Dashboard", Message::ChangeScreen(Screen::Dashboard)),
        ("Meals", Message::ChangeScreen(Screen::MealList)),
        (
            "Create Product",
            Message::ChangeScreen(Screen::CreateProduct),
        ),
        ("Product List", Message::ChangeScreen(Screen::ProductList)),
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

    let today = Local::now().date_naive();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

    let formatted_day = match app.day {
        d if d == today => "Today".to_string(),
        d if d == tomorrow => "Tomorrow".to_string(),
        d if d == yesterday => "Yesterday".to_string(),
        _ => app.day.format("%Y-%m-%d").to_string(),
    };
    let day_row = row![
        Button::new("<").on_press(Message::PrevDay),
        horizontal_space(),
        Text::new(formatted_day).size(20),
        horizontal_space(),
        Button::new(">").on_press(Message::NextDay),
    ]
    .align_y(Center)
    .width(200)
    .spacing(10);

    column![navigation, vertical_space(), day_row].into()
}
