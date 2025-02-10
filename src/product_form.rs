use iced::{widget::column, Element};

use crate::{
    app::Message,
    data::CreateUpdateProduct,
    form_field::{render_input_form_field, InputFormField, InputFormFieldError},
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

    pub fn new_filled(
        name: &str,
        company: &str,
        calories: &str,
        fats: &str,
        proteins: &str,
        carbohydrates: &str,
    ) -> Self {
        CreateUpdateProductForm {
            name: InputFormField::new_with_raw_value("Name*", "Chicken", name),
            company: InputFormField::new_with_raw_value("Company", "Chicken Inc.", company),
            calories: InputFormField::new_with_raw_value("Calories* (kcal)", "100.0", calories),
            fats: InputFormField::new_with_raw_value("Fats* (g)", "2.0", fats),
            proteins: InputFormField::new_with_raw_value("Proteins* (g)", "20.0", proteins),
            carbohydrates: InputFormField::new_with_raw_value(
                "Carbohydrates* (g)",
                "1.0",
                carbohydrates,
            ),
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
        render_input_form_field(&form.name, |s| Message::UpdateCreateProductFormName(s)),
        render_input_form_field(&form.company, |s| Message::UpdateCreateProductFormCompany(
            s
        )),
        render_input_form_field(
            &form.calories,
            |s| Message::UpdateCreateProductFormCalories(s)
        ),
        render_input_form_field(&form.fats, |s| Message::UpdateCreateProductFormFats(s)),
        render_input_form_field(
            &form.proteins,
            |s| Message::UpdateCreateProductFormProteins(s)
        ),
        render_input_form_field(&form.carbohydrates, |s| {
            Message::UpdateCreateProductFormCarbohydrates(s)
        }),
    ]
    .spacing(10)
    .into()
}
