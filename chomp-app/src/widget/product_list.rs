use chomp_services::Product;
use iced::{
    widget::{button, column, row, Button, Container, Scrollable, Text},
    Alignment, Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, style::TableRowStyle, InputFormField, Widget};

#[derive(Debug, Clone)]
pub enum ProductListMessage {
    RedirectToCreate,
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
    filtered_products: Vec<Product>,
}

impl ProductList {
    pub fn new(products: Vec<Product>) -> Self {
        ProductList {
            name_filter: InputFormField::new("Product search", "Chicken"),
            products: products.clone(),
            filtered_products: products,
        }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.products = ctx.services.product.list().unwrap_or_default();
        self.filter();
    }

    fn filter(&mut self) {
        let name_filter = self.name_filter.raw_input.clone();
        if name_filter.is_empty() {
            self.filtered_products = self.products.clone();
        } else {
            self.filtered_products = self
                .products
                .iter()
                .filter(|p| {
                    p.name
                        .to_lowercase()
                        .contains(name_filter.to_lowercase().as_str())
                })
                .cloned()
                .collect();
        }
    }
}

impl Widget for ProductList {
    fn view(&self) -> Element<Message> {
        let mut table = column![list_header_row()];
        for (i, product) in self.filtered_products.iter().enumerate() {
            table = table.push(list_row(product, i % 2 == 0))
        }

        let content = column![
            row![
                Text::new("Product list").size(40),
                Button::new("+").on_press(ProductListMessage::RedirectToCreate.into())
            ]
            .spacing(10)
            .align_y(Alignment::Center),
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

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::ProductList(msg) = msg {
            match msg {
                ProductListMessage::RedirectToCreate => {
                    ctx.next_widget = Some(NextWidget::CreateProduct);
                }
                ProductListMessage::ProductSearch(s) => {
                    self.name_filter.raw_input = s;
                    self.filter();
                }
                ProductListMessage::DeleteProduct(product_id) => {
                    if let Err(err) = ctx.services.product.delete(product_id) {
                        tracing::error!("Failed to delete product: {}", err);
                        std::process::exit(1);
                    }
                    self.refresh(ctx);
                }
            }
        };

        Task::none()
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
            Button::new("Delete")
                .style(button::danger)
                .on_press(ProductListMessage::DeleteProduct(p.id).into())
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
