use std::collections::HashSet;

use chrono::{Days, Local, NaiveDate};
use iced::{
    widget::{column, combo_box, container, horizontal_space, row, Button, Text},
    Alignment::Center,
    Element, Length,
};

use crate::{
    app::Message,
    data::{AddMealProduct, Meal, MealProduct, Product, UpdateMealProductWeight},
    form_field::{render_input_form_field, InputFormField, InputFormFieldError},
    meal_list_screen::MealListMessage,
};

#[derive(Debug)]
pub struct MealProductForm {
    pub combo_box_state: combo_box::State<Product>,
    pub combo_box_error: Option<InputFormFieldError>,
    pub weight: InputFormField<f64>,
    pub meal: Meal,
    pub product_id: Option<usize>,
}

impl MealProductForm {
    pub fn new(products: &[Product], meal: &Meal) -> Self {
        let meal_product_names: HashSet<String> =
            meal.products.iter().map(|mp| mp.name.clone()).collect();

        let mut products_not_in_meal: Vec<Product> = products
            .iter()
            .filter(|p| !meal_product_names.contains(&p.name))
            .cloned()
            .collect();

        products_not_in_meal.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        MealProductForm {
            combo_box_state: combo_box::State::new(products_not_in_meal),
            combo_box_error: None,
            weight: InputFormField::new("Weight (g)", "20.0"),
            meal: meal.to_owned(),
            product_id: None,
        }
    }

    pub fn parse(&mut self) -> Result<AddMealProduct, String> {
        self.weight.validate(|input| {
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

        if self.product_id.is_none() {
            self.combo_box_error = Some(InputFormFieldError::MissingRequiredValue);
        }

        Ok(AddMealProduct {
            meal_id: self.meal.id,
            product_id: self.product_id.ok_or("validation failed")?,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

pub fn render_add_product_to_meal_form(form: &MealProductForm) -> Element<Message> {
    let selected_product = match &form.product_id {
        Some(id) => form.combo_box_state.options().iter().find(|p| p.id == *id),
        None => None,
    };

    let combo_box = combo_box(
        &form.combo_box_state,
        "Search product...",
        selected_product,
        |p| MealListMessage::CreateMealProductFormProduct(p.id).into(),
    )
    .width(Length::Fill);

    container(
        column![
            Text::new(format!("Add product to {}", form.meal.name)).size(30),
            combo_box,
            render_input_form_field(&form.weight, |w| {
                MealListMessage::CreateMealProductFormWeight(w).into()
            }),
            Button::new("Add Product")
                .width(Length::Fill)
                .on_press(MealListMessage::SubmitAddMealProductForm.into()),
            Button::new("Cancel")
                .width(Length::Fill)
                .on_press(MealListMessage::CreateMealProductFormMeal(None).into())
        ]
        .spacing(10),
    )
    .width(300)
    .padding(30)
    .style(container::rounded_box)
    .into()
}

#[derive(Debug)]
pub struct UpdateMealProductForm {
    pub meal_product: MealProduct,
    pub weight: InputFormField<f64>,
}

impl UpdateMealProductForm {
    pub fn new(meal_product: &MealProduct) -> Self {
        UpdateMealProductForm {
            meal_product: meal_product.clone(),
            weight: InputFormField::new_with_raw_value(
                "Weight (g)",
                "20.0",
                &meal_product.weight.to_string(),
            ),
        }
    }

    pub fn parse(&mut self) -> Result<UpdateMealProductWeight, String> {
        self.weight.validate(|input| {
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

        Ok(UpdateMealProductWeight {
            meal_product_id: self.meal_product.id,
            weight: self.weight.value.ok_or("validation failed")?,
        })
    }
}

pub fn render_update_meal_product_form(form: &UpdateMealProductForm) -> Element<Message> {
    container(
        column![
            Text::new(format!("Edit weight of {}", form.meal_product.name)).size(30),
            render_input_form_field(&form.weight, |w| {
                MealListMessage::UpdateMealProductFormWeight(w).into()
            }),
            Button::new("Update Weight")
                .width(Length::Fill)
                .on_press(MealListMessage::SubmitUpdateMealProductForm.into()),
            Button::new("Cancel")
                .width(Length::Fill)
                .on_press(MealListMessage::UpdateMealProductFormMealProduct(None).into())
        ]
        .spacing(10),
    )
    .width(300)
    .padding(30)
    .style(container::rounded_box)
    .into()
}

#[derive(Debug)]
pub struct CopyMealProductsForm {
    pub target_meal: Meal,
    pub meal_products: Vec<MealProduct>,
    pub from_day: NaiveDate,
}

impl CopyMealProductsForm {
    pub fn new(meal_products: &Vec<MealProduct>, from_day: &NaiveDate, to_meal: &Meal) -> Self {
        CopyMealProductsForm {
            target_meal: to_meal.to_owned(),
            meal_products: meal_products.to_owned(),
            from_day: from_day.to_owned(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AddMealProduct>, String> {
        Ok(self
            .meal_products
            .iter()
            .map(|mp| AddMealProduct {
                meal_id: self.target_meal.id,
                product_id: mp.product_id,
                weight: mp.weight,
            })
            .collect())
    }
}

pub fn render_copy_meal_products_form(form: &CopyMealProductsForm) -> Element<Message> {
    let today = Local::now().date_naive();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

    let formatted_from_day = match form.from_day {
        d if d == today => "Today".to_string(),
        d if d == tomorrow => "Tomorrow".to_string(),
        d if d == yesterday => "Yesterday".to_string(),
        d => d.format("%Y-%m-%d").to_string(),
    };

    let formatted_target_day = match form.target_meal.day {
        d if d == today => "Today".to_string(),
        d if d == tomorrow => "Tomorrow".to_string(),
        d if d == yesterday => "Yesterday".to_string(),
        d => d.format("%Y-%m-%d").to_string(),
    };

    let day_row = row![
        Button::new("<").on_press(
            MealListMessage::CopyMealProductsFromDay(
                form.from_day.checked_sub_days(Days::new(1)).unwrap()
            )
            .into()
        ),
        horizontal_space(),
        Text::new(formatted_from_day.clone()).size(20),
        horizontal_space(),
        Button::new(">").on_press(
            MealListMessage::CopyMealProductsFromDay(
                form.from_day.checked_add_days(Days::new(1)).unwrap()
            )
            .into()
        ),
    ]
    .align_y(Center)
    .width(Length::Fill)
    .spacing(10);

    container(
        column![
            Text::new(format!(
                "Copy {} products to {} {}",
                form.meal_products.len(),
                formatted_target_day,
                form.target_meal.name
            ))
            .size(30),
            day_row,
            Button::new("Copy Meal")
                .width(Length::Fill)
                .on_press(MealListMessage::SubmitCopyMealProductsForm.into()),
            Button::new("Cancel")
                .width(Length::Fill)
                .on_press(MealListMessage::CopyMealProductsMeal(None).into())
        ]
        .spacing(10),
    )
    .width(300)
    .padding(30)
    .style(container::rounded_box)
    .into()
}
