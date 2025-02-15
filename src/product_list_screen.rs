use iced::{
    widget::{column, row, Button, Container, Scrollable, Text},
    Element, Length,
};

use crate::{
    app::{App, Message, Screen},
    data::Product,
    sidebar::render_sidebar,
    style::TableRowStyle,
};

#[derive(Debug, Clone)]
pub enum ProductListMessage {
    DeleteProduct(usize),
}

impl From<ProductListMessage> for Message {
    fn from(value: ProductListMessage) -> Self {
        Message::ProductList(value)
    }
}

pub fn render_product_list_screen(app: &App) -> Element<Message> {
    let mut table = column![list_header_row()];
    for (i, product) in app.products.iter().enumerate() {
        table = table.push(list_row(product, i % 2 == 0))
    }

    let content = column![Text::new("Product list").size(40), Scrollable::new(table)].spacing(10);

    row![render_sidebar(app), content]
        .height(Length::Fill)
        .padding(20)
        .spacing(20)
        .into()
}

fn list_header_row() -> Element<'static, Message> {
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
        Text::new(format!("{:.1}", p.calories)).width(Length::Fill),
        Text::new(format!("{:.1}", p.fats)).width(Length::Fill),
        Text::new(format!("{:.1}", p.proteins)).width(Length::Fill),
        Text::new(format!("{:.1}", p.carbohydrates)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::ChangeScreen(Screen::UpdateProduct(p.id))),
            Button::new("Delete").on_press(ProductListMessage::DeleteProduct(p.id).into())
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
