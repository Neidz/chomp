use iced::{
    widget::{column, row, Button, Text},
    Element, Length,
};

use crate::{
    app::{App, Message},
    data::CreateUpdateProduct,
    form_field::{render_input_form_field, InputFormField, InputFormFieldError},
    sidebar::render_sidebar,
};

#[derive(Debug, Clone)]
pub enum CreateProductMessage {
    UpdateFormName(String),
    UpdateFormCompany(String),
    UpdateFormCalories(String),
    UpdateFormFats(String),
    UpdateFormProteins(String),
    UpdateFormCarbohydrates(String),
    SubmitForm,
}

impl From<CreateProductMessage> for Message {
    fn from(value: CreateProductMessage) -> Self {
        Message::CreateProduct(value)
    }
}

pub fn render_create_product_screen(app: &App) -> Element<Message> {
    let form = app.create_product_form.as_ref().unwrap();

    let content = column![
        Text::new("Create product").size(40),
        form.render(),
        Button::new("Create").on_press(CreateProductMessage::SubmitForm.into())
    ]
    .spacing(10);

    row![render_sidebar(app), content]
        .height(Length::Fill)
        .padding(20)
        .spacing(20)
        .into()
}

#[derive(Debug)]
pub struct CreateProductForm {
    pub name: InputFormField<String>,
    pub company: InputFormField<Option<String>>,
    pub calories: InputFormField<f64>,
    pub fats: InputFormField<f64>,
    pub proteins: InputFormField<f64>,
    pub carbohydrates: InputFormField<f64>,
}

impl CreateProductForm {
    pub fn new() -> Self {
        CreateProductForm {
            name: InputFormField::new("Name*", "Chicken"),
            company: InputFormField::new("Company", "Chicken Inc."),
            calories: InputFormField::new("Calories* (kcal)", "100.0"),
            fats: InputFormField::new("Fats* (g)", "2.0"),
            proteins: InputFormField::new("Proteins* (g)", "20.0"),
            carbohydrates: InputFormField::new("Carbohydrates* (g)", "1.0"),
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
                match input.parse::<f64>() {
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
                match input.parse::<f64>() {
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
                match input.parse::<f64>() {
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
                match input.parse::<f64>() {
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

    fn render(&self) -> Element<Message> {
        column![
            render_input_form_field(&self.name, |n| {
                CreateProductMessage::UpdateFormName(n).into()
            }),
            render_input_form_field(&self.company, |c| CreateProductMessage::UpdateFormCompany(
                c
            )
            .into()),
            render_input_form_field(&self.calories, |c| {
                CreateProductMessage::UpdateFormCalories(c).into()
            }),
            render_input_form_field(&self.fats, |f| CreateProductMessage::UpdateFormFats(f)
                .into()),
            render_input_form_field(&self.proteins, |p| {
                CreateProductMessage::UpdateFormProteins(p).into()
            }),
            render_input_form_field(&self.carbohydrates, |c| {
                CreateProductMessage::UpdateFormCarbohydrates(c).into()
            }),
        ]
        .spacing(10)
        .into()
    }
}
