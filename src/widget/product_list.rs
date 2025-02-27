use iced::{
    widget::{column, row, Button, Container, Scrollable, Text},
    Element, Length,
};

use crate::{
    app::{Context, Message, NextWidget},
    data::Product,
    style::TableRowStyle,
};

use super::{form_field::InputFormField, sidebar::sidebar, Widget};

#[derive(Debug, Clone)]
pub enum ProductListMessage {
    ProductSearch(String),
    DeleteProduct(usize),
}

impl From<ProductListMessage> for Message {
    fn from(value: ProductListMessage) -> Self {
        Message::ProductList(value)
    }
}

#[derive(Debug)]
pub struct ProductList {
    name_filter: InputFormField<String>,
    products: Vec<Product>,
}

impl ProductList {
    pub fn new(products: Vec<Product>) -> Self {
        ProductList {
            name_filter: InputFormField::new("Product search", "Chicken"),
            products,
        }
    }

    fn refresh(&mut self, ctx: &Context) {
        let name_filter = if self.name_filter.raw_input.is_empty() {
            None
        } else {
            Some(self.name_filter.raw_input.as_str())
        };
        self.products = ctx.data.product.list(name_filter).unwrap();
    }
}

impl Widget for ProductList {
    fn view(&self) -> Element<Message> {
        let mut table = column![list_header_row()];
        for (i, product) in self.products.iter().enumerate() {
            table = table.push(list_row(product, i % 2 == 0))
        }

        let content = column![
            Text::new("Product list").size(40),
            self.name_filter
                .view(|s| ProductListMessage::ProductSearch(s).into()),
            Scrollable::new(table)
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) {
        if let Message::ProductList(msg) = msg {
            match msg {
                ProductListMessage::ProductSearch(s) => {
                    self.name_filter.raw_input = s;
                    self.refresh(ctx);
                }
                ProductListMessage::DeleteProduct(product_id) => {
                    ctx.data.product.delete(product_id).unwrap();
                    self.refresh(ctx);
                }
            }
        }
    }
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
            Button::new("Update").on_press(Message::ChangeWidget(NextWidget::UpdateProduct(p.id))),
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
