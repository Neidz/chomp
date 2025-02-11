use core::panic;

use chrono::{Local, NaiveDate};
use iced::{
    widget::{center, column, container, mouse_area, opaque, row, stack, Button, Column, Text},
    Color, Element, Length,
};
use rusqlite::Connection;

use crate::{
    data::{Data, DataError, Meal, Product},
    form_field::InputFormFieldError,
    meal_list::render_meal_list,
    meal_product_form::{
        render_add_product_to_meal_form, render_update_meal_product_form, MealProductForm,
        UpdateMealProductForm,
    },
    product_form::{render_product_form, CreateUpdateProductForm},
    product_list::render_product_list,
};

type MealId = usize;
type MealProductId = usize;
type ProductId = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    CreateProduct,
    UpdateProduct(ProductId),
    ProductList,
    MealList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),

    UpdateCreateProductFormName(String),
    UpdateCreateProductFormCompany(String),
    UpdateCreateProductFormCalories(String),
    UpdateCreateProductFormFats(String),
    UpdateCreateProductFormProteins(String),
    UpdateCreateProductFormCarbohydrates(String),
    SubmitCreateProductForm,
    SubmitUpdateProductForm,

    DeleteProduct(ProductId),

    CreateMealProductFormMeal(Option<MealId>),
    CreateMealProductFormWeight(String),
    CreateMealProductFormProduct(ProductId),
    SubmitAddMealProductForm,

    UpdateMealProductFormMealProduct(Option<MealProductId>),
    UpdateMealProductFormWeight(String),
    SubmitUpdateMealProductForm,

    DeleteMealProduct(MealProductId),
}

pub struct App {
    data: Data,
    screen: Screen,
    day: NaiveDate,

    products: Vec<Product>,
    meals: Vec<Meal>,

    create_product_form: CreateUpdateProductForm,
    update_product_form: Option<(ProductId, CreateUpdateProductForm)>,
    add_meal_product_form: Option<MealProductForm>,
    update_meal_product_form: Option<UpdateMealProductForm>,
}

impl App {
    pub fn new(db: Connection) -> Self {
        let day = Local::now().date_naive();
        let data = Data::new(db);
        let products = data.product.list().unwrap();
        let meals = data.meal.list_or_create_default(day).unwrap();

        App {
            data,
            screen: Screen::Home,
            day,
            products: products.clone(),
            meals,
            create_product_form: CreateUpdateProductForm::new(),
            update_product_form: None,
            add_meal_product_form: None,
            update_meal_product_form: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Home => self.home_screen(),
            Screen::CreateProduct => self.create_product_screen(),
            Screen::ProductList => self.product_list_screen(),
            Screen::MealList => self.meals_screen(),
            Screen::UpdateProduct(_) => self.update_product_screen(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeScreen(s) => match s {
                Screen::UpdateProduct(product_id) => {
                    let p = self.data.product.read(product_id).unwrap();
                    let form = CreateUpdateProductForm::new_filled(
                        &p.name,
                        &p.company.unwrap_or("".to_string()),
                        &p.calories.to_string(),
                        &p.fats.to_string(),
                        &p.proteins.to_string(),
                        &p.carbohydrates.to_string(),
                    );

                    self.update_product_form = Some((product_id, form));
                    self.screen = Screen::UpdateProduct(product_id);
                }
                _ => self.screen = s,
            },
            Message::UpdateCreateProductFormName(name) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.name.raw_input = name;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.name.raw_input = name;
                }
            }
            Message::UpdateCreateProductFormCompany(company) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.company.raw_input = company;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.company.raw_input = company;
                }
            }
            Message::UpdateCreateProductFormCalories(raw_calories) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.calories.raw_input = raw_calories;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.calories.raw_input = raw_calories;
                }
            }
            Message::UpdateCreateProductFormFats(raw_fats) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.fats.raw_input = raw_fats;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.fats.raw_input = raw_fats;
                }
            }
            Message::UpdateCreateProductFormProteins(raw_proteins) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.proteins.raw_input = raw_proteins;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.proteins.raw_input = raw_proteins;
                }
            }
            Message::UpdateCreateProductFormCarbohydrates(raw_carbohydrates) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.carbohydrates.raw_input = raw_carbohydrates;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.carbohydrates.raw_input = raw_carbohydrates;
                }
            }
            Message::SubmitCreateProductForm => {
                if let Ok(product) = self.create_product_form.parse() {
                    if let Some(err) = self.data.product.create(product).err() {
                        match err {
                            DataError::UniqueConstraintViolation(unique_field)
                                if unique_field == "products.name" =>
                            {
                                self.create_product_form.name.error =
                                    Some(InputFormFieldError::Custom(
                                        "Product with this name already exists".to_string(),
                                    ))
                            }
                            _ => {
                                eprintln!("Error: {:?}", err);
                            }
                        }
                    } else {
                        self.create_product_form.reset();
                        self.refresh_products();
                        self.screen = Screen::ProductList
                    }
                };
            }
            Message::SubmitUpdateProductForm => {
                let (product_id, form) = &mut self.update_product_form.as_mut().unwrap();

                if let Ok(product) = form.parse() {
                    if let Some(err) = self.data.product.update(*product_id, product).err() {
                        match err {
                            DataError::UniqueConstraintViolation(unique_field)
                                if unique_field == "products.name" =>
                            {
                                form.name.error = Some(InputFormFieldError::Custom(
                                    "Product with this name already exists".to_string(),
                                ))
                            }
                            _ => {
                                eprintln!("Error: {:?}", err);
                            }
                        }
                    } else {
                        form.reset();
                        self.refresh_products();
                        self.screen = Screen::ProductList
                    }
                };
            }
            Message::DeleteProduct(product_id) => {
                self.data.product.delete(product_id).unwrap();
                self.refresh_products();
            }
            Message::CreateMealProductFormMeal(meal_id) => match meal_id {
                Some(id) => {
                    let meal = self.data.meal.read(id).unwrap();
                    self.add_meal_product_form = Some(MealProductForm::new(&self.products, &meal));
                }
                None => {
                    self.add_meal_product_form = None;
                }
            },
            Message::CreateMealProductFormWeight(raw_weight) => {
                let form = &mut self.add_meal_product_form.as_mut().unwrap();
                form.weight.raw_input = raw_weight;
            }
            Message::CreateMealProductFormProduct(product_id) => {
                let form = &mut self.add_meal_product_form.as_mut().unwrap();
                form.product_id = Some(product_id);
            }
            Message::SubmitAddMealProductForm => {
                if let Ok(add_meal_product) = self.add_meal_product_form.as_mut().unwrap().parse() {
                    self.data.meal.add_product(add_meal_product).unwrap();

                    self.refresh_meals();
                    self.add_meal_product_form = None;
                }
            }
            Message::UpdateMealProductFormMealProduct(meal_product_id) => match meal_product_id {
                Some(id) => {
                    let meal_product = self.data.meal.read_product(id).unwrap();
                    self.update_meal_product_form = Some(UpdateMealProductForm::new(&meal_product));
                }
                None => {
                    self.update_meal_product_form = None;
                }
            },
            Message::UpdateMealProductFormWeight(raw_weight) => {
                let form = &mut self.update_meal_product_form.as_mut().unwrap();
                form.weight.raw_input = raw_weight;
            }
            Message::SubmitUpdateMealProductForm => {
                if let Ok(update_meal_product_weight) =
                    self.update_meal_product_form.as_mut().unwrap().parse()
                {
                    self.data
                        .meal
                        .update_product_weight(update_meal_product_weight)
                        .unwrap();

                    self.refresh_meals();
                    self.update_meal_product_form = None;
                }
            }
            Message::DeleteMealProduct(meal_product_id) => {
                self.data.meal.delete_product(meal_product_id).unwrap();
                self.refresh_meals();
            }
        }
    }

    fn home_screen(&self) -> Element<Message> {
        let content = column![Text::new("Home").size(40)].spacing(10);

        row![self.sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn create_product_screen(&self) -> Element<Message> {
        let content = column![
            Text::new("Create product").size(40),
            render_product_form(&self.create_product_form),
            Button::new("Create").on_press(Message::SubmitCreateProductForm)
        ]
        .spacing(10);

        row![self.sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn update_product_screen(&self) -> Element<Message> {
        let (_id, form) = self.update_product_form.as_ref().unwrap();

        let content = column![
            Text::new("Update product").size(40),
            render_product_form(form),
            Button::new("Update").on_press(Message::SubmitUpdateProductForm)
        ]
        .spacing(10);

        row![self.sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn product_list_screen(&self) -> Element<Message> {
        let content = column![
            Text::new("Product list").size(40),
            render_product_list(&self.products)
        ]
        .spacing(10);

        row![self.sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .into()
    }

    fn meals_screen(&self) -> Element<Message> {
        let content =
            column![Text::new("Meals").size(40), render_meal_list(&self.meals)].spacing(10);

        let content_with_sidebar = row![self.sidebar(), content]
            .height(Length::Fill)
            .padding(20)
            .spacing(20);

        match (&self.add_meal_product_form, &self.update_meal_product_form) {
            (Some(add_form), None) => self.modal(
                content_with_sidebar.into(),
                render_add_product_to_meal_form(add_form),
                Message::CreateMealProductFormMeal(None),
            ),
            (None, Some(update_form)) => self.modal(
                content_with_sidebar.into(),
                render_update_meal_product_form(update_form),
                Message::UpdateMealProductFormMealProduct(None),
            ),
            (Some(_), Some(_)) => panic!("Both add and update meal product forms found"),
            (None, None) => content_with_sidebar.into(),
        }
    }

    fn refresh_products(&mut self) {
        let products = self.data.product.list().unwrap();
        self.products = products.clone();
    }

    fn refresh_meals(&mut self) {
        self.meals = self.data.meal.list_or_create_default(self.day).unwrap();
    }

    fn modal<'a>(
        &self,
        base: Element<'a, Message>,
        modal_content: Element<'a, Message>,
        on_blur: Message,
    ) -> Element<'a, Message> {
        stack![
            base,
            opaque(
                mouse_area(center(opaque(modal_content)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),

                        ..container::Style::default()
                    }
                }))
                .on_press(on_blur)
            )
        ]
        .into()
    }

    fn sidebar(&self) -> Element<Message> {
        let buttons = vec![
            ("Home", Message::ChangeScreen(Screen::Home)),
            ("Meals", Message::ChangeScreen(Screen::MealList)),
            (
                "Create Product",
                Message::ChangeScreen(Screen::CreateProduct),
            ),
            ("Product List", Message::ChangeScreen(Screen::ProductList)),
        ];

        buttons
            .into_iter()
            .map(|(content, message)| {
                Button::new(content)
                    .on_press(message)
                    .width(Length::Fill)
                    .into()
            })
            .collect::<Column<Message>>()
            .width(200)
            .spacing(10)
            .into()
    }
}
