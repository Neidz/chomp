use std::collections::HashSet;

use chomp_services::{
    AddMealProduct, Meal, MealDayStats, MealProduct, NutritionTarget, Product,
    UpdateMealProductWeight,
};
use chrono::{Days, NaiveDate};
use iced::{
    widget::{
        button, column, combo_box, container, horizontal_space, progress_bar, row, vertical_space,
        Button, Container, Scrollable, Text,
    },
    Alignment, Element, Length, Task,
};

use crate::app::{Context, Message};

use super::{
    modal, sidebar, style::TableRowStyle, DatePicker, InputFormField, InputFormFieldError, Widget,
};

#[derive(Debug, Clone)]
pub enum MealListMessage {
    CreateMealProductFormMeal(Option<usize>),
    CreateMealProductFormWeight(String),
    CreateMealProductFormProduct(usize),
    SubmitAddMealProductForm,

    UpdateMealProductFormMealProduct(Option<usize>),
    UpdateMealProductFormWeight(String),
    SubmitUpdateMealProductForm,

    DeleteMealProduct(usize),

    CopyMealProductsMeal(Option<usize>),
    SubmitCopyMealProductsForm,
}

impl From<MealListMessage> for Message {
    fn from(value: MealListMessage) -> Self {
        Message::MealList(value)
    }
}

#[derive(Debug)]
pub struct MealList {
    day: DatePicker,
    meals: Vec<Meal>,
    stats: MealDayStats,
    target: NutritionTarget,

    add_meal_product_form: Option<MealProductForm>,
    update_meal_product_form: Option<UpdateMealProductForm>,
    copy_meal_products_form: Option<CopyMealProductsForm>,
}

impl MealList {
    pub fn new(
        day: NaiveDate,
        meals: Vec<Meal>,
        stats: MealDayStats,
        target: NutritionTarget,
    ) -> Self {
        assert!(!meals.is_empty());
        MealList {
            day: DatePicker::new_with_value("Date", &day),
            meals,
            stats,
            target,
            add_meal_product_form: None,
            update_meal_product_form: None,
            copy_meal_products_form: None,
        }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.meals = match ctx.services.meal.list_or_create_default(self.day.value()) {
            Ok(m) => m,
            Err(err) => {
                tracing::error!("Failed to get list of meals: {}", err);
                std::process::exit(1);
            }
        };
        self.stats = match ctx.services.meal.day_stats(self.day.value()) {
            Ok(s) => s,
            Err(err) => {
                tracing::error!("Failed to get day stats: {}", err);
                std::process::exit(1);
            }
        };
    }
}

impl Widget for MealList {
    fn view(&self) -> Element<Message> {
        let mut tables = column![].spacing(20);
        for meal in self.meals.iter() {
            tables = tables.push(render_meal(meal))
        }

        let content = column![
            row![
                Text::new("Meals").size(40),
                horizontal_space(),
                self.day.view(),
            ]
            .align_y(Alignment::Center),
            Scrollable::new(tables),
            vertical_space(),
            meal_stats(&self.stats, &self.target)
        ]
        .spacing(10);

        let content_with_sidebar = row![sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20);

        if let Some(add_form) = &self.add_meal_product_form {
            return modal(
                content_with_sidebar.into(),
                render_add_product_to_meal_form(add_form),
                MealListMessage::CreateMealProductFormMeal(None).into(),
                true,
            );
        }

        if let Some(update_form) = &self.update_meal_product_form {
            return modal(
                content_with_sidebar.into(),
                render_update_meal_product_form(update_form),
                MealListMessage::UpdateMealProductFormMealProduct(None).into(),
                true,
            );
        }

        if let Some(copy_form) = &self.copy_meal_products_form {
            return modal(
                modal(
                    content_with_sidebar.into(),
                    render_copy_meal_products_form(copy_form),
                    MealListMessage::CopyMealProductsMeal(None).into(),
                    true,
                ),
                self.day.view_modal(),
                Message::CloseDatePicker,
                self.day.calendar_open(),
            );
        }

        modal(
            content_with_sidebar.into(),
            self.day.view_modal(),
            Message::CloseDatePicker,
            self.day.calendar_open(),
        )
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message> {
        if let Some(form) = self.copy_meal_products_form.as_mut() {
            form.from_day.handle_message(msg.clone());
        } else {
            self.day.handle_message(msg.clone());
        }

        match msg {
            Message::DatePickerDateChange(new_day) => {
                if let Some(form) = self.copy_meal_products_form.as_mut() {
                    let new_products = ctx
                        .services
                        .meal
                        .read_by_day_and_name(new_day, &form.target_meal.name)
                        .map(|m| m.products)
                        .unwrap_or_default();

                    form.meal_products = new_products;
                } else if self.add_meal_product_form.is_none()
                    && self.update_meal_product_form.is_none()
                {
                    self.refresh(ctx);
                };
            }
            Message::MealList(msg) => match msg {
                MealListMessage::CreateMealProductFormMeal(meal_id) => match meal_id {
                    Some(id) => {
                        let meal = match ctx.services.meal.read(id) {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!("Failed to get meal: {}", err);
                                std::process::exit(1);
                            }
                        };
                        let products = ctx.services.product.list().unwrap_or_default();
                        self.add_meal_product_form = Some(MealProductForm::new(products, &meal));
                    }
                    None => {
                        self.add_meal_product_form = None;
                    }
                },

                MealListMessage::CreateMealProductFormWeight(raw_weight) => {
                    let form = self.add_meal_product_form.as_mut().unwrap();
                    form.weight.raw_input = raw_weight;
                }
                MealListMessage::CreateMealProductFormProduct(product_id) => {
                    let form = self.add_meal_product_form.as_mut().unwrap();
                    form.product_id = Some(product_id);
                }
                MealListMessage::SubmitAddMealProductForm => {
                    match self.add_meal_product_form.as_mut().unwrap().parse() {
                        Ok(add_meal_product) => {
                            if let Err(err) = ctx.services.meal.add_product(add_meal_product) {
                                tracing::error!("Failed to add product: {}", err);
                                std::process::exit(1);
                            }
                            self.refresh(ctx);
                            self.add_meal_product_form = None;
                        }
                        Err(err) => {
                            tracing::warn!("Failed to parse add meal product form: {}", err)
                        }
                    }
                }
                MealListMessage::UpdateMealProductFormMealProduct(meal_product_id) => {
                    match meal_product_id {
                        Some(id) => {
                            let meal_product = match ctx.services.meal.read_product(id) {
                                Ok(mp) => mp,
                                Err(err) => {
                                    tracing::error!("Failed to get meal product: {}", err);
                                    std::process::exit(1);
                                }
                            };
                            self.update_meal_product_form =
                                Some(UpdateMealProductForm::new(&meal_product));
                        }
                        None => {
                            self.update_meal_product_form = None;
                        }
                    }
                }
                MealListMessage::UpdateMealProductFormWeight(raw_weight) => {
                    let form = self.update_meal_product_form.as_mut().unwrap();
                    form.weight.raw_input = raw_weight;
                }
                MealListMessage::SubmitUpdateMealProductForm => {
                    match self.update_meal_product_form.as_mut().unwrap().parse() {
                        Ok(update_meal_product_weight) => {
                            ctx.services
                                .meal
                                .update_product_weight(update_meal_product_weight)
                                .unwrap();

                            self.refresh(ctx);
                            self.update_meal_product_form = None;
                        }
                        Err(err) => {
                            tracing::warn!("Failed to parse update meal product form: {}", err)
                        }
                    }
                }
                MealListMessage::DeleteMealProduct(meal_product_id) => {
                    if let Err(err) = ctx.services.meal.delete_product(meal_product_id) {
                        tracing::error!("Failed to delete meal product: {}", err);
                        std::process::exit(1);
                    }
                    self.refresh(ctx);
                }
                MealListMessage::CopyMealProductsMeal(meal_id) => match meal_id {
                    Some(id) => {
                        let meal = match ctx.services.meal.read(id) {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!("Failed to get meal: {}", err);
                                std::process::exit(1);
                            }
                        };
                        let prev_day = meal.day.checked_sub_days(Days::new(1)).unwrap();
                        let prev_day_meal =
                            ctx.services.meal.read_by_day_and_name(prev_day, &meal.name);

                        let meal_products = prev_day_meal.map(|m| m.products).unwrap_or_default();

                        self.copy_meal_products_form =
                            Some(CopyMealProductsForm::new(&meal_products, &prev_day, &meal));
                    }
                    None => self.copy_meal_products_form = None,
                },
                MealListMessage::SubmitCopyMealProductsForm => {
                    match self.copy_meal_products_form.as_mut().unwrap().parse() {
                        Ok(add_meal_products) => {
                            add_meal_products.into_iter().for_each(|add_meal_product| {
                                if let Err(err) = ctx.services.meal.add_product(add_meal_product) {
                                    tracing::error!(
                                        "Failed to add meal product while copying meal: {}",
                                        err
                                    );
                                    std::process::exit(1);
                                }
                            });
                            self.refresh(ctx);
                            self.copy_meal_products_form = None;
                        }
                        Err(err) => {
                            tracing::warn!(
                                "Failed to parse form for copying meal products: {}",
                                err
                            );
                        }
                    }
                }
            },
            Message::EscapeClicked => {
                self.update_meal_product_form = None;
                self.copy_meal_products_form = None;
                self.add_meal_product_form = None;
            }
            _ => {}
        }

        Task::none()
    }
}

fn render_meal(meal: &Meal) -> Element<Message> {
    let mut table = column![
        row![
            Text::new(&meal.name).size(20),
            Button::new("Add Product")
                .on_press(MealListMessage::CreateMealProductFormMeal(Some(meal.id)).into()),
            Button::new("Copy From Different Day")
                .on_press(MealListMessage::CopyMealProductsMeal(Some(meal.id)).into())
        ]
        .spacing(10),
        list_header_row()
    ];

    let mut calories_sum = 0f32;
    let mut fats_sum = 0f32;
    let mut proteins_sum = 0f32;
    let mut carbohydrates_sum = 0f32;

    for (i, meal_product) in meal.products.iter().enumerate() {
        calories_sum += meal_product.calories;
        fats_sum += meal_product.fats;
        proteins_sum += meal_product.proteins;
        carbohydrates_sum += meal_product.carbohydrates;

        table = table.push(list_row(meal_product, i % 2 == 0));
    }

    if !meal.products.is_empty() {
        table = table.push(list_footer(
            calories_sum,
            fats_sum,
            proteins_sum,
            carbohydrates_sum,
        ));
    }

    table.into()
}

fn list_header_row() -> Element<'static, Message> {
    let row = row![
        Text::new("Name").width(Length::Fill),
        Text::new("Weight (g)").width(Length::Fill),
        Text::new("Calories (kcal)").width(Length::Fill),
        Text::new("Fats (g)").width(Length::Fill),
        Text::new("Proteins (g)").width(Length::Fill),
        Text::new("Carbohydrates (g)").width(Length::Fill),
        Text::new("Actions").width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row).width(Length::Fill).into()
}

fn list_row(mp: &MealProduct, even: bool) -> Element<Message> {
    let row = row![
        Text::new(&mp.name).width(Length::Fill),
        Text::new(format!("{:.1}", mp.weight)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.calories)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.fats)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.proteins)).width(Length::Fill),
        Text::new(format!("{:.1}", mp.carbohydrates)).width(Length::Fill),
        row![
            Button::new("Update")
                .on_press(MealListMessage::UpdateMealProductFormMealProduct(Some(mp.id)).into()),
            Button::new("Delete")
                .style(button::danger)
                .on_press(MealListMessage::DeleteMealProduct(mp.id).into())
        ]
        .spacing(10)
        .width(Length::Fill)
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row)
        .width(Length::Fill)
        .style(move |t| {
            if even {
                TableRowStyle::Even.style(t)
            } else {
                TableRowStyle::Odd.style(t)
            }
        })
        .into()
}

fn list_footer(
    calories_sum: f32,
    fats_sum: f32,
    proteins_sum: f32,
    carbohydrates_sum: f32,
) -> Element<'static, Message> {
    let row = row![
        Text::new("Sum").width(Length::Fill),
        Text::new("").width(Length::Fill),
        Text::new(format!("{calories_sum:.1}")).width(Length::Fill),
        Text::new(format!("{fats_sum:.1}")).width(Length::Fill),
        Text::new(format!("{proteins_sum:.1}")).width(Length::Fill),
        Text::new(format!("{carbohydrates_sum:.1}")).width(Length::Fill),
        Text::new("").width(Length::Fill),
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row)
        .style(|t| TableRowStyle::Footer.style(t))
        .width(Length::Fill)
        .into()
}

pub fn meal_stats(stats: &MealDayStats, target: &NutritionTarget) -> Element<'static, Message> {
    row![
        meal_stat("Calories", stats.calories, target.calories),
        meal_stat("Proteins", stats.proteins, target.proteins),
        meal_stat("Fats", stats.fats, target.fats),
        meal_stat("Carbohydrates", stats.carbohydrates, target.carbohydrates)
    ]
    .spacing(40)
    .width(Length::Fill)
    .into()
}

fn meal_stat(label: &str, value: f32, max_value: f32) -> Element<Message> {
    column![
        Text::new(format!("{label} {value:.1}/{max_value:.1}")),
        progress_bar(0.0..=100.0, value / max_value * 100.0),
    ]
    .align_x(Alignment::Center)
    .spacing(2)
    .into()
}

#[derive(Debug)]
pub struct MealProductForm {
    pub combo_box_state: combo_box::State<Product>,
    pub combo_box_error: Option<InputFormFieldError>,
    pub weight: InputFormField<f32>,
    pub meal: Meal,
    pub product_id: Option<usize>,
}

impl MealProductForm {
    pub fn new(products: Vec<Product>, meal: &Meal) -> Self {
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
                match input.parse::<f32>() {
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
            form.weight
                .view(|w| { MealListMessage::CreateMealProductFormWeight(w).into() }),
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
    pub weight: InputFormField<f32>,
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
                match input.parse::<f32>() {
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
            form.weight
                .view(|w| { MealListMessage::UpdateMealProductFormWeight(w).into() }),
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
    pub from_day: DatePicker,
}

impl CopyMealProductsForm {
    pub fn new(meal_products: &Vec<MealProduct>, from_day: &NaiveDate, to_meal: &Meal) -> Self {
        CopyMealProductsForm {
            target_meal: to_meal.to_owned(),
            meal_products: meal_products.to_owned(),
            from_day: DatePicker::new_with_value("From date", from_day),
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
    container(
        column![
            Text::new(format!(
                "Copy {} products to {}",
                form.meal_products.len(),
                form.target_meal.name
            ))
            .size(30),
            form.from_day.view(),
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
