use std::collections::HashSet;

use chrono::{Days, Local, NaiveDate};
use iced::{
    widget::{
        column, combo_box, container, horizontal_space, progress_bar, row, vertical_space, Button,
        Container, Scrollable, Text,
    },
    Alignment, Element, Length,
};

use crate::{
    app::{Context, Message},
    data::{
        AddMealProduct, CalorieTarget, Meal, MealDayStats, MealProduct, Product,
        UpdateMealProductWeight,
    },
};

use super::{
    modal::modal, sidebar::sidebar, style::TableRowStyle, InputFormField, InputFormFieldError,
    Widget,
};

#[derive(Debug, Clone)]
pub enum MealListMessage {
    NextDay,
    PrevDay,

    CreateMealProductFormMeal(Option<usize>),
    CreateMealProductFormWeight(String),
    CreateMealProductFormProduct(usize),
    SubmitAddMealProductForm,

    UpdateMealProductFormMealProduct(Option<usize>),
    UpdateMealProductFormWeight(String),
    SubmitUpdateMealProductForm,

    DeleteMealProduct(usize),

    CopyMealProductsMeal(Option<usize>),
    CopyMealProductsFromDay(NaiveDate),
    SubmitCopyMealProductsForm,
}

impl From<MealListMessage> for Message {
    fn from(value: MealListMessage) -> Self {
        Message::MealList(value)
    }
}

#[derive(Debug)]
pub struct MealList {
    day: NaiveDate,
    meals: Vec<Meal>,
    stats: MealDayStats,
    target: CalorieTarget,

    add_meal_product_form: Option<MealProductForm>,
    update_meal_product_form: Option<UpdateMealProductForm>,
    copy_meal_products_form: Option<CopyMealProductsForm>,
}

impl MealList {
    pub fn new(
        day: NaiveDate,
        meals: Vec<Meal>,
        stats: MealDayStats,
        target: CalorieTarget,
    ) -> Self {
        assert!(!meals.is_empty());
        MealList {
            day,
            meals,
            stats,
            target,
            add_meal_product_form: None,
            update_meal_product_form: None,
            copy_meal_products_form: None,
        }
    }

    fn refresh(&mut self, ctx: &Context) {
        self.meals = match ctx.data.meal.list_or_create_default(self.day) {
            Ok(m) => m,
            Err(err) => {
                tracing::error!("Failed to get list of meals: {}", err);
                panic!();
            }
        };
        self.stats = match ctx.data.meal.day_stats(self.day) {
            Ok(s) => s,
            Err(err) => {
                tracing::error!("Failed to get day stats: {}", err);
                panic!();
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
                day_changer(self.day)
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
            );
        }

        if let Some(update_form) = &self.update_meal_product_form {
            return modal(
                content_with_sidebar.into(),
                render_update_meal_product_form(update_form),
                MealListMessage::UpdateMealProductFormMealProduct(None).into(),
            );
        }

        if let Some(copy_form) = &self.copy_meal_products_form {
            return modal(
                content_with_sidebar.into(),
                render_copy_meal_products_form(copy_form),
                MealListMessage::CopyMealProductsMeal(None).into(),
            );
        }

        content_with_sidebar.into()
    }

    fn update(&mut self, ctx: &mut Context, msg: Message) {
        match msg {
            Message::MealList(msg) => match msg {
                MealListMessage::NextDay => {
                    self.day = self.day.checked_add_days(Days::new(1)).unwrap();
                    self.refresh(ctx);
                }
                MealListMessage::PrevDay => {
                    self.day = self.day.checked_sub_days(Days::new(1)).unwrap();
                    self.refresh(ctx);
                }
                MealListMessage::CreateMealProductFormMeal(meal_id) => match meal_id {
                    Some(id) => {
                        let meal = match ctx.data.meal.read(id) {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!("Failed to get meal: {}", err);
                                panic!();
                            }
                        };
                        let products = ctx.data.product.list().unwrap_or_default();
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
                            if let Err(err) = ctx.data.meal.add_product(add_meal_product) {
                                tracing::error!("Failed to add product: {}", err);
                                panic!();
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
                            let meal_product = match ctx.data.meal.read_product(id) {
                                Ok(mp) => mp,
                                Err(err) => {
                                    tracing::error!("Failed to get meal product: {}", err);
                                    panic!();
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
                            ctx.data
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
                    if let Err(err) = ctx.data.meal.delete_product(meal_product_id) {
                        tracing::error!("Failed to delete meal product: {}", err);
                        panic!();
                    }
                    self.refresh(ctx);
                }
                MealListMessage::CopyMealProductsMeal(meal_id) => match meal_id {
                    Some(id) => {
                        let meal = match ctx.data.meal.read(id) {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!("Failed to get meal: {}", err);
                                panic!();
                            }
                        };
                        let prev_day = meal.day.checked_sub_days(Days::new(1)).unwrap();
                        let prev_day_meal =
                            ctx.data.meal.read_by_day_and_name(prev_day, &meal.name);

                        let meal_products = prev_day_meal.map(|m| m.products).unwrap_or_default();

                        self.copy_meal_products_form =
                            Some(CopyMealProductsForm::new(&meal_products, &prev_day, &meal));
                    }
                    None => self.copy_meal_products_form = None,
                },
                MealListMessage::CopyMealProductsFromDay(new_day) => {
                    let form = self.copy_meal_products_form.as_mut().unwrap();

                    let new_products = ctx
                        .data
                        .meal
                        .read_by_day_and_name(new_day, &form.target_meal.name)
                        .map(|m| m.products)
                        .unwrap_or_default();

                    form.from_day = new_day;
                    form.meal_products = new_products;
                }
                MealListMessage::SubmitCopyMealProductsForm => {
                    match self.copy_meal_products_form.as_mut().unwrap().parse() {
                        Ok(add_meal_products) => {
                            add_meal_products.into_iter().for_each(|add_meal_product| {
                                if let Err(err) = ctx.data.meal.add_product(add_meal_product) {
                                    tracing::error!(
                                        "Failed to add meal product while copying meal: {}",
                                        err
                                    );
                                    panic!();
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
    }
}

fn day_changer(day: NaiveDate) -> Element<'static, Message> {
    let today = Local::now().date_naive();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let yesterday = today.checked_sub_days(Days::new(1)).unwrap();

    let formatted_day = match day {
        d if d == today => "Today".to_string(),
        d if d == tomorrow => "Tomorrow".to_string(),
        d if d == yesterday => "Yesterday".to_string(),
        _ => day.format("%Y-%m-%d").to_string(),
    };

    row![
        Button::new("<").on_press(MealListMessage::PrevDay.into()),
        horizontal_space(),
        Text::new(formatted_day).size(20),
        horizontal_space(),
        Button::new(">").on_press(MealListMessage::NextDay.into()),
    ]
    .align_y(Alignment::Center)
    .width(220)
    .spacing(10)
    .into()
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
            Button::new("Delete").on_press(MealListMessage::DeleteMealProduct(mp.id).into())
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
        Text::new(format!("{:.1}", calories_sum)).width(Length::Fill),
        Text::new(format!("{:.1}", fats_sum)).width(Length::Fill),
        Text::new(format!("{:.1}", proteins_sum)).width(Length::Fill),
        Text::new(format!("{:.1}", carbohydrates_sum)).width(Length::Fill),
        Text::new("").width(Length::Fill),
    ]
    .padding(10)
    .width(Length::Fill);

    Container::new(row)
        .style(|t| TableRowStyle::Footer.style(t))
        .width(Length::Fill)
        .into()
}

pub fn meal_stats(stats: &MealDayStats, target: &CalorieTarget) -> Element<'static, Message> {
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
        Text::new(format!("{} {:.1}/{:.1}", label, value, max_value)),
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
    .align_y(Alignment::Center)
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
