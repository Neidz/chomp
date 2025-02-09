use iced::{
    widget::{column, row, Button, Container, Scrollable, Text},
    Element, Length,
};

use crate::{
    app::{Message, Screen},
    data::Product,
    style::TableRowStyle,
};

pub fn render_product_list(products: &[Product]) -> Element<Message> {
    let mut table = column![list_header()];
    for (i, product) in products.iter().enumerate() {
        table = table.push(list_row(product, i % 2 == 0))
    }

    Scrollable::new(table).into()
}

fn list_header() -> Element<'static, Message> {
    let row = row![
        Text::new("Id").width(Length::Fill),
        Text::new("Name").width(Length::Fill),
        Text::new("Company").width(Length::Fill),
        Text::new("Calories (kcal/100g)").width(Length::Fill),
        Text::new("Fats (g/100kcal)").width(Length::Fill),
        Text::new("Proteins (g/100g)").width(Length::Fill),
        Text::new("Carbohydrates (g/100g)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row(p: &Product, even: bool) -> Element<Message> {
    let row = row![
        Text::new(format!("{}", p.id)).width(Length::Fill),
        Text::new(&p.name).width(Length::Fill),
        Text::new(p.company.as_deref().unwrap_or("-")).width(Length::Fill),
        Text::new(format!("{:.2}", p.calories)).width(Length::Fill),
        Text::new(format!("{:.2}", p.fats)).width(Length::Fill),
        Text::new(format!("{:.2}", p.proteins)).width(Length::Fill),
        Text::new(format!("{:.2}", p.carbohydrates)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::ChangeScreen(Screen::UpdateProduct(p.id))),
            Button::new("Delete").on_press(Message::DeleteProduct(p.id))
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
