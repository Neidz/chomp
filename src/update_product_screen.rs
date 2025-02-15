use iced::{
    widget::{column, row, Button, Text},
    Element, Length,
};

use crate::{
    app::{App, Message},
    data::{CreateUpdateProduct, Product},
    form_field::{render_input_form_field, InputFormField, InputFormFieldError},
    sidebar::render_sidebar,
};

#[derive(Debug, Clone)]
pub enum UpdateProductMessage {
    UpdateFormName(String),
    UpdateFormCompany(String),
    UpdateFormCalories(String),
    UpdateFormFats(String),
    UpdateFormProteins(String),
    UpdateFormCarbohydrates(String),
    SubmitForm,
}

impl From<UpdateProductMessage> for Message {
    fn from(value: UpdateProductMessage) -> Self {
        Message::UpdateProduct(value)
    }
}

pub fn render_update_product_screen(app: &App) -> Element<Message> {
    let form = app.update_product_form.as_ref().unwrap();

    let content = column![
        Text::new("Update product").size(40),
        form.render(),
        Button::new("Create").on_press(UpdateProductMessage::SubmitForm.into())
    ]
    .spacing(10);

    row![render_sidebar(app), content]
        .height(Length::Fill)
        .padding(20)
        .spacing(20)
        .into()
}

#[derive(Debug)]
pub struct UpdateProductForm {
    pub product_id: usize,
    pub name: InputFormField<String>,
    pub company: InputFormField<Option<String>>,
    pub calories: InputFormField<f64>,
    pub fats: InputFormField<f64>,
    pub proteins: InputFormField<f64>,
    pub carbohydrates: InputFormField<f64>,
}

impl UpdateProductForm {
    pub fn new(p: &Product) -> Self {
        UpdateProductForm {
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

    pub fn render(&self) -> Element<Message> {
        column![
            render_input_form_field(&self.name, |n| {
                UpdateProductMessage::UpdateFormName(n).into()
            }),
            render_input_form_field(&self.company, |c| UpdateProductMessage::UpdateFormCompany(
                c
            )
            .into()),
            render_input_form_field(&self.calories, |c| {
                UpdateProductMessage::UpdateFormCalories(c).into()
            }),
            render_input_form_field(&self.fats, |f| UpdateProductMessage::UpdateFormFats(f)
                .into()),
            render_input_form_field(&self.proteins, |p| {
                UpdateProductMessage::UpdateFormProteins(p).into()
            }),
            render_input_form_field(&self.carbohydrates, |c| {
                UpdateProductMessage::UpdateFormCarbohydrates(c).into()
            }),
        ]
        .spacing(10)
        .into()
    }
}
