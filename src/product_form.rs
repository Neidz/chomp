use iced::{
    widget::{column, Button, Text, TextInput},
    Color, Element,
};

use crate::{
    app::Message,
    data::CreateUpdateProduct,
    form_field::{InputFormField, InputFormFieldError},
};

#[derive(Debug)]
pub struct CreateUpdateProductForm {
    pub name: InputFormField<String>,
    pub company: InputFormField<Option<String>>,
    pub calories: InputFormField<f64>,
    pub fats: InputFormField<f64>,
    pub proteins: InputFormField<f64>,
    pub carbohydrates: InputFormField<f64>,
}

impl CreateUpdateProductForm {
    pub fn new() -> Self {
        CreateUpdateProductForm {
            name: InputFormField::new("Name*", "Chicken"),
            company: InputFormField::new("Company", "Chicken Inc."),
            calories: InputFormField::new("Calories* (kcal)", "100.0"),
            fats: InputFormField::new("Fats* (g)", "2.0"),
            proteins: InputFormField::new("Proteins* (g)", "20.0"),
            carbohydrates: InputFormField::new("Carbohydrates* (g)", "1.0"),
        }
    }

    pub fn reset(&mut self) {
        *self = CreateUpdateProductForm::new();
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
}

pub fn render_product_form(form: &CreateUpdateProductForm) -> Element<Message> {
    column![
        render_field(&form.name, |s| Message::UpdateCreateProductFormName(s)),
        render_field(&form.company, |s| Message::UpdateCreateProductFormCompany(
            s
        )),
        render_field(
            &form.calories,
            |s| Message::UpdateCreateProductFormCalories(s)
        ),
        render_field(&form.fats, |s| Message::UpdateCreateProductFormFats(s)),
        render_field(
            &form.proteins,
            |s| Message::UpdateCreateProductFormProteins(s)
        ),
        render_field(&form.carbohydrates, |s| {
            Message::UpdateCreateProductFormCarbohydrates(s)
        }),
        Button::new("Create").on_press(Message::SubmitCreateProductForm)
    ]
    .spacing(10)
    .into()
}

fn render_field<T, F>(field: &InputFormField<T>, handle_message: F) -> Element<Message>
where
    F: Fn(String) -> Message + 'static,
{
    let mut column = column![
        Text::new(&field.name),
        TextInput::new(&field.placeholder, &field.raw_input).on_input(handle_message)
    ];

    if let Some(err) = &field.error {
        column = column.push(Text::new(err.to_string()).color(Color::from_rgb(1.0, 0.0, 0.0)));
    }

    column.into()
}
