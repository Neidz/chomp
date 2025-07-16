use chomp_services::{Product, ServiceError};
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum CreateProductPortionMessage {
    UpdateName(String),
    UpdateWeight(String),
    Submit,
}

impl From<CreateProductPortionMessage> for Message {
    fn from(value: CreateProductPortionMessage) -> Self {
        Message::CreateProductPortion(value)
    }
}

#[derive(Debug)]
pub struct CreateProductPortion {
    product: Product,
    name: InputFormField<String>,
    weight: InputFormField<f32>,
}

impl CreateProductPortion {
    pub fn new(product: &Product) -> Self {
        CreateProductPortion {
            product: product.to_owned(),
            name: InputFormField::new("Name*", "One package"),
            weight: InputFormField::new("Weight (g)*", "100.0"),
        }
    }

    pub fn parse(&mut self) -> Result<chomp_services::CreateProductPortion, String> {
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

        Ok(chomp_services::CreateProductPortion {
            product_id: self.product.id,
            name: self.name.value.clone().ok_or("validation failed")?,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for CreateProductPortion {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.name
                .view(|n| { CreateProductPortionMessage::UpdateName(n).into() }),
            self.weight
                .view(|w| { CreateProductPortionMessage::UpdateWeight(w).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new(format!("Create portion for {}", self.product.name)).size(40),
            form,
            Button::new("Create").on_press(CreateProductPortionMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::CreateProductPortion(msg) = msg {
            match msg {
                CreateProductPortionMessage::UpdateName(name) => {
                    self.name.raw_input = name;
                }
                CreateProductPortionMessage::UpdateWeight(raw_weight) => {
                    self.weight.raw_input = raw_weight;
                }

                CreateProductPortionMessage::Submit => {
                    if let Ok(portion) = self.parse() {
                        if let Some(err) = ctx.services.product_portion.create(portion).err() {
                            match err {
                                ServiceError::UniqueConstraintViolation(unique_field)
                                    if unique_field
                                        == "product_portions.name, product_portions.product_id" =>
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
