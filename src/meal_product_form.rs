use std::collections::HashSet;

use iced::{
    widget::{column, combo_box, container, Button, Text},
    Element, Length,
};

use crate::{
    app::Message,
    data::{AddMealProduct, Meal, MealProduct, Product, UpdateMealProductWeight},
    form_field::{render_input_form_field, InputFormField, InputFormFieldError},
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
        let meal_product_ids: HashSet<usize> = meal.products.iter().map(|mp| mp.id).collect();

        let mut products_not_in_meal: Vec<Product> = products
            .iter()
            .filter(|p| !meal_product_ids.contains(&p.id))
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
        |p| Message::CreateMealProductFormProduct(p.id),
    )
    .width(Length::Fill);

    container(
        column![
            Text::new(format!("Add product to {}", form.meal.name)).size(30),
            combo_box,
            render_input_form_field(&form.weight, |w| Message::CreateMealProductFormWeight(w)),
            Button::new("Add Product")
                .width(Length::Fill)
                .on_press(Message::SubmitAddMealProductForm),
            Button::new("Cancel")
                .width(Length::Fill)
                .on_press(Message::CreateMealProductFormMeal(None))
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
            render_input_form_field(&form.weight, |w| Message::UpdateMealProductFormWeight(w)),
            Button::new("Update weight")
                .width(Length::Fill)
                .on_press(Message::SubmitUpdateMealProductForm),
            Button::new("Cancel")
                .width(Length::Fill)
                .on_press(Message::UpdateMealProductFormMealProduct(None))
        ]
        .spacing(10),
    )
    .width(300)
    .padding(30)
    .style(container::rounded_box)
    .into()
}
