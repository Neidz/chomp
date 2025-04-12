use iced::{
    widget::{column, row, Button, Text},
    Element, Length, Task,
};

use crate::{
    app::{Context, Message, NextWidget},
    data::{CreateUpdateProduct, DataError},
};

use super::{sidebar::sidebar, InputFormField, InputFormFieldError, Widget};

#[derive(Debug, Clone)]
pub enum CreateProductMessage {
    UpdateName(String),
    UpdateCompany(String),
    UpdateCalories(String),
    UpdateFats(String),
    UpdateProteins(String),
    UpdateCarbohydrates(String),
    Submit,
}

impl From<CreateProductMessage> for Message {
    fn from(value: CreateProductMessage) -> Self {
        Message::CreateProduct(value)
    }
}

#[derive(Debug)]
pub struct CreateProduct {
    name: InputFormField<String>,
    company: InputFormField<Option<String>>,
    calories: InputFormField<f32>,
    fats: InputFormField<f32>,
    proteins: InputFormField<f32>,
    carbohydrates: InputFormField<f32>,
}

impl CreateProduct {
    pub fn new() -> Self {
        CreateProduct {
            name: InputFormField::new("Name*", "Chicken"),
            company: InputFormField::new("Company", "Chicken Inc."),
            calories: InputFormField::new("Calories* (kcal/100g)", "100.0"),
            fats: InputFormField::new("Fats* (g/100g)", "2.0"),
            proteins: InputFormField::new("Proteins* (g/100g)", "20.0"),
            carbohydrates: InputFormField::new("Carbohydrates* (g/100g)", "1.0"),
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

impl Widget for CreateProduct {
    fn view(&self) -> Element<Message> {
        let form = column![
            self.name
                .view(|n| { CreateProductMessage::UpdateName(n).into() }),
            self.company
                .view(|c| { CreateProductMessage::UpdateCompany(c).into() }),
            self.calories
                .view(|c| { CreateProductMessage::UpdateCalories(c).into() }),
            self.fats
                .view(|f| { CreateProductMessage::UpdateFats(f).into() }),
            self.proteins
                .view(|p| { CreateProductMessage::UpdateProteins(p).into() }),
            self.carbohydrates
                .view(|c| { CreateProductMessage::UpdateCarbohydrates(c).into() }),
        ]
        .spacing(10);

        let content = column![
            Text::new("Create product").size(40),
            form,
            Button::new("Create").on_press(CreateProductMessage::Submit.into())
        ]
        .spacing(10);

        row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Message::CreateProduct(msg) = msg {
            match msg {
                CreateProductMessage::UpdateName(name) => {
                    self.name.raw_input = name;
                }
                CreateProductMessage::UpdateCompany(company) => {
                    self.company.raw_input = company;
                }
                CreateProductMessage::UpdateCalories(raw_calories) => {
                    self.calories.raw_input = raw_calories;
                }
                CreateProductMessage::UpdateFats(raw_fats) => {
                    self.fats.raw_input = raw_fats;
                }
                CreateProductMessage::UpdateProteins(raw_proteins) => {
                    self.proteins.raw_input = raw_proteins;
                }
                CreateProductMessage::UpdateCarbohydrates(raw_carbohydrates) => {
                    self.carbohydrates.raw_input = raw_carbohydrates;
                }
                CreateProductMessage::Submit => {
                    if let Ok(product) = self.parse() {
                        if let Some(err) = ctx.data.product.create(product).err() {
                            match err {
                                DataError::UniqueConstraintViolation(unique_field)
                                    if unique_field == "products.name" =>
                                {
                                    self.name.error = Some(InputFormFieldError::Custom(
                                        "Product with this name already exists".to_string(),
                                    ))
                                }
                                _ => {
                                    eprintln!("Error: {:?}", err);
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
