use chomp_services::{Product, ProductPortion, ServiceError};
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum UpdateProductPortionMessage {
    UpdateName(String),
    UpdateWeight(String),
    Submit,
}

impl From<UpdateProductPortionMessage> for Message {
    fn from(value: UpdateProductPortionMessage) -> Self {
        Message::UpdateProductPortion(value)
    }
}

#[derive(Debug)]
pub struct UpdateProductPortion {
    product_portion_id: usize,
    product: Product,
    name: InputFormField<String>,
    weight: InputFormField<f32>,
}

impl UpdateProductPortion {
    pub fn new(product: &Product, product_portion: &ProductPortion) -> Self {
        UpdateProductPortion {
            product_portion_id: product_portion.id,
            product: product.to_owned(),
            name: InputFormField::new_with_raw_value("Name*", "One package", &product_portion.name),
            weight: InputFormField::new_with_raw_value(
                "Weight (g)*",
                "100.0",
                &product_portion.weight.to_string(),
            ),
        }
    }

    pub fn parse(&mut self) -> Result<ProductPortion, String> {
        self.name.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else if input.len() < 3 {
                Err(InputFormFieldError::TooShort(3))
            } else {
                Ok(input.to_string())
            }
        });

        self.weight.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else {
                match input.parse::<f32>() {
                    Err(_) => Err(InputFormFieldError::InvalidNumber),
                    Ok(val) if val < 0.0 => Err(InputFormFieldError::SmallerThanZero),
                    Ok(val) => Ok(val),
                }
            }
        });

        Ok(ProductPortion {
            id: self.product_portion_id,
            product_id: self.product.id,
            name: self.name.value.clone().ok_or("validation failed")?,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for UpdateProductPortion {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.name
                .view(|n| { UpdateProductPortionMessage::UpdateName(n).into() }),
            self.weight
                .view(|w| { UpdateProductPortionMessage::UpdateWeight(w).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new(format!("Update portion for {}", self.product.name)).size(40),
            form,
            Button::new("Update").on_press(UpdateProductPortionMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::UpdateProductPortion(msg) = msg {
            match msg {
                UpdateProductPortionMessage::UpdateName(name) => {
                    self.name.raw_input = name;
                }
                UpdateProductPortionMessage::UpdateWeight(raw_weight) => {
                    self.weight.raw_input = raw_weight;
                }

                UpdateProductPortionMessage::Submit => {
                    if let Ok(portion) = self.parse() {
                        if let Some(err) = ctx.services.product_portion.update(portion).err() {
                            match err {
                                ServiceError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "product_portions.name" =>
                                {
                                    self.name.error = Some(InputFormFieldError::Custom(
                                        "Portion with this name already exists for this product"
                                            .to_string(),
                                    ))
                                }
                                _ => {
                                    eprintln!("Error: {err:?}");
                                }
                            }
                        } else {
                            ctx.next_widget = Some(NextWidget::ProductPortionList(self.product.id));
                        }
                    };
                }
            }
        };

        Task::none()
    }
}
