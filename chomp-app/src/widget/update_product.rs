use chomp_services::{CreateUpdateProduct, Product, ServiceError};
use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::app::{Context, Message, NextWidget};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum UpdateProductMessage {
    UpdateName(String),
    UpdateCompany(String),
    UpdateCalories(String),
    UpdateFats(String),
    UpdateProteins(String),
    UpdateCarbohydrates(String),
    Submit,
}

impl From<UpdateProductMessage> for Message {
    fn from(value: UpdateProductMessage) -> Self {
        Message::UpdateProduct(value)
    }
}

#[derive(Debug)]
pub struct UpdateProduct {
    product_id: usize,
    name: InputFormField<String>,
    company: InputFormField<Option<String>>,
    calories: InputFormField<f32>,
    fats: InputFormField<f32>,
    proteins: InputFormField<f32>,
    carbohydrates: InputFormField<f32>,
}

impl UpdateProduct {
    pub fn new(p: Product) -> Self {
        UpdateProduct {
            product_id: p.id,
            name: InputFormField::new_with_raw_value("Name*", "Chicken", &p.name),
            company: InputFormField::new_with_raw_value(
                "Company",
                "Chicken Inc.",
                &p.company.clone().unwrap_or("".to_string()),
            ),
            calories: InputFormField::new_with_raw_value(
                "Calories* (kcal)",
                "100.0",
                &p.calories.to_string(),
            ),
            fats: InputFormField::new_with_raw_value("Fats* (g)", "2.0", &p.fats.to_string()),
            proteins: InputFormField::new_with_raw_value(
                "Proteins* (g)",
                "20.0",
                &p.proteins.to_string(),
            ),
            carbohydrates: InputFormField::new_with_raw_value(
                "Carbohydrates* (g)",
                "1.0",
                &p.carbohydrates.to_string(),
            ),
        }
    }

    pub fn parse(&mut self) -> Result<CreateUpdateProduct, String> {
        self.name.validate(|input| {
            if input.is_empty() {
                Err(InputFormFieldError::MissingRequiredValue)
            } else if input.len() < 3 {
                Err(InputFormFieldError::TooShort(3))
            } else {
                Ok(input.to_string())
            }
        });

        self.company.validate(|input| {
            if input.is_empty() {
                Ok(None)
            } else {
                Ok(Some(input.to_string()))
            }
        });

        self.calories.validate(|input| {
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

        self.fats.validate(|input| {
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

        self.proteins.validate(|input| {
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

        self.carbohydrates.validate(|input| {
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

        Ok(CreateUpdateProduct {
            name: self.name.value.clone().ok_or("validation failed")?,
            company: self.company.value.clone().ok_or("validation failed")?,
            calories: self.calories.value.ok_or("validation failed")?,
            fats: self.fats.value.ok_or("validation failed")?,
            proteins: self.proteins.value.ok_or("validation failed")?,
            carbohydrates: self.carbohydrates.value.ok_or("validation failed")?,
        })
    }
}

impl Widget for UpdateProduct {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.name
                .view(|n| { UpdateProductMessage::UpdateName(n).into() }),
            self.company
                .view(|c| { UpdateProductMessage::UpdateCompany(c).into() }),
            self.calories
                .view(|c| { UpdateProductMessage::UpdateCalories(c).into() }),
            self.fats
                .view(|f| { UpdateProductMessage::UpdateFats(f).into() }),
            self.proteins
                .view(|p| { UpdateProductMessage::UpdateProteins(p).into() }),
            self.carbohydrates
                .view(|c| { UpdateProductMessage::UpdateCarbohydrates(c).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new("Update product").size(40),
            form,
            Button::new("Update").on_press(UpdateProductMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::UpdateProduct(msg) = msg {
            match msg {
                UpdateProductMessage::UpdateName(name) => {
                    self.name.raw_input = name;
                }
                UpdateProductMessage::UpdateCompany(company) => {
                    self.company.raw_input = company;
                }
                UpdateProductMessage::UpdateCalories(raw_calories) => {
                    self.calories.raw_input = raw_calories;
                }
                UpdateProductMessage::UpdateFats(raw_fats) => {
                    self.fats.raw_input = raw_fats;
                }
                UpdateProductMessage::UpdateProteins(raw_proteins) => {
                    self.proteins.raw_input = raw_proteins;
                }
                UpdateProductMessage::UpdateCarbohydrates(raw_carbohydrates) => {
                    self.carbohydrates.raw_input = raw_carbohydrates;
                }
                UpdateProductMessage::Submit => {
                    if let Ok(product) = self.parse() {
                        if let Some(err) =
                            ctx.services.product.update(self.product_id, product).err()
                        {
                            match err {
                                ServiceError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "products.name" =>
                                {
                                    self.name.error = Some(InputFormFieldError::Custom(
                                        "Product with this name already exists".to_string(),
                                    ))
                                }
                                _ => {
                                    eprintln!("Error: {err:?}");
                                }
                            }
                        } else {
                            ctx.next_widget = Some(NextWidget::ProductList);
                        }
                    };
                }
            }
        };

        Task::none()
    }
}
