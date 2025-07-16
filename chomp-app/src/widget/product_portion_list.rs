use chomp_services::{Product, ProductPortion};
use iced::{
    widget::{button, column, row, Button, Container, Scrollable, Text},
    Alignment, Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar, style::TableRowStyle, Widget};

type ProductPortionId = usize;

#[derive(Debug, Clone)]
pub enum ProductPortionListMessage {
    RedirectToCreate,
    DeleteProductPortion(ProductPortionId),
}

impl From<ProductPortionListMessage> for Message {
    fn from(value: ProductPortionListMessage) -> Self {
        Message::ProductPortionList(value)
    }
}

#[derive(Debug)]
pub struct ProductPortionList {
    product: Product,
    portions: Vec<ProductPortion>,
}

impl ProductPortionList {
    pub fn new(product: Product, portions: Vec<ProductPortion>) -> Self {
        ProductPortionList { product, portions }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.portions = ctx
            .services
            .product_portion
            .list(self.product.id)
            .unwrap_or_default();
    }
}

impl Widget for ProductPortionList {
    fn view(&self) -> Element<Message> {
        let mut table = column![list_header_row()];
        for (i, portion) in self.portions.iter().enumerate() {
            table = table.push(list_row(&self.product, portion, i % 2 == 0))
        }

        let content = column![
            row![
                Text::new(format!("Portions for {}", self.product.name)).size(40),
                Button::new("+").on_press(ProductPortionListMessage::RedirectToCreate.into())
            ]
            .spacing(10)
            .align_y(Alignment::Center),
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
        if let Message::ProductPortionList(msg) = msg {
            match msg {
                ProductPortionListMessage::RedirectToCreate => {
                    ctx.next_widget = Some(NextWidget::CreateProductPortion(self.product.id));
                }
                ProductPortionListMessage::DeleteProductPortion(portion_id) => {
                    if let Err(err) = ctx.services.product_portion.delete(portion_id) {
                        tracing::error!("Failed to delete product portion: {}", err);
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
        Text::new("Name").width(Length::Fill),
        Text::new("Weight (g)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row<'a>(
    product: &'a Product,
    portion: &'a ProductPortion,
    even: bool,
) -> Element<'a, Message> {
    let row = row![
        Text::new(&portion.name).width(Length::Fill),
        Text::new(format!("{:.1}", portion.weight)).width(Length::Fill),
        row![
            Button::new("Update").on_press(Message::ChangeWidget(
                NextWidget::UpdateProductPortion(product.id, portion.id)
            )),
            Button::new("Delete")
                .style(button::danger)
                .on_press(ProductPortionListMessage::DeleteProductPortion(portion.id).into())
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
