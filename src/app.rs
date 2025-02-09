use chrono::{Local, NaiveDate};
use iced::{
    widget::{column, row, Button, Column, Text},
    Element, Length,
};
use rusqlite::Connection;

use crate::{
    data::{Data, DataError, Meal, Product},
    form_field::InputFormFieldError,
    meal_list::render_meal_list,
    product_form::{render_product_form, CreateUpdateProductForm},
    product_list::render_product_list,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    CreateProduct,
    UpdateProduct(usize),
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
    DeleteProduct(usize),
}

pub struct App {
    data: Data,
    screen: Screen,
    day: NaiveDate,

    products: Vec<Product>,
    meals: Vec<Meal>,

    create_product_form: CreateUpdateProductForm,
    update_product_form: Option<(usize, CreateUpdateProductForm)>,
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
            products,
            meals,
            create_product_form: CreateUpdateProductForm::new(),
            update_product_form: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = match self.screen {
            Screen::Home => self.home_screen(),
            Screen::CreateProduct => self.create_product_screen(),
            Screen::ProductList => self.product_list_screen(),
            Screen::MealList => self.meals_screen(),
            Screen::UpdateProduct(_) => self.update_product_screen(),
        };

        row![self.sidebar(), content].padding(20).spacing(20).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeScreen(s) => match s {
                Screen::UpdateProduct(id) => {
                    let p = self.data.product.read(id).unwrap();
                    let form = CreateUpdateProductForm::new_filled(
                        &p.name,
                        &p.company.unwrap_or("".to_string()),
                        &p.calories.to_string(),
                        &p.fats.to_string(),
                        &p.proteins.to_string(),
                        &p.carbohydrates.to_string(),
                    );

                    self.update_product_form = Some((id, form));
                    self.screen = Screen::UpdateProduct(id);
                }
                _ => self.screen = s,
            },
            Message::UpdateCreateProductFormName(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.name.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.name.raw_input = s;
                }
            }
            Message::UpdateCreateProductFormCompany(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.company.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.company.raw_input = s;
                }
            }
            Message::UpdateCreateProductFormCalories(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.calories.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.calories.raw_input = s;
                }
            }
            Message::UpdateCreateProductFormFats(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.fats.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.fats.raw_input = s;
                }
            }
            Message::UpdateCreateProductFormProteins(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.proteins.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.proteins.raw_input = s;
                }
            }
            Message::UpdateCreateProductFormCarbohydrates(s) => {
                if self.screen == Screen::CreateProduct {
                    self.create_product_form.carbohydrates.raw_input = s;
                } else {
                    let (_, form) = &mut self.update_product_form.as_mut().unwrap();
                    form.carbohydrates.raw_input = s;
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
                let (id, form) = &mut self.update_product_form.as_mut().unwrap();

                if let Ok(product) = form.parse() {
                    if let Some(err) = self.data.product.update(*id, product).err() {
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
            Message::DeleteProduct(id) => {
                self.data.product.delete(id).unwrap();
                self.refresh_products();
            }
        }
    }

    fn home_screen(&self) -> Element<Message> {
        column![Text::new("Home").size(40)].spacing(10).into()
    }

    fn create_product_screen(&self) -> Element<Message> {
        column![
            Text::new("Create Product").size(40),
            render_product_form(&self.create_product_form),
            Button::new("Create").on_press(Message::SubmitCreateProductForm)
        ]
        .spacing(10)
        .into()
    }

    fn update_product_screen(&self) -> Element<Message> {
        let (_id, form) = self.update_product_form.as_ref().unwrap();

        column![
            Text::new("Update Product").size(40),
            render_product_form(form),
            Button::new("Update").on_press(Message::SubmitUpdateProductForm)
        ]
        .spacing(10)
        .into()
    }

    fn product_list_screen(&self) -> Element<Message> {
        column![
            Text::new("Product List").size(40),
            render_product_list(&self.products)
        ]
        .spacing(10)
        .into()
    }

    fn meals_screen(&self) -> Element<Message> {
        column![Text::new("Meals").size(40), render_meal_list(&self.meals)]
            .spacing(10)
            .into()
    }

    fn refresh_products(&mut self) {
        self.products = self.data.product.list().unwrap();
    }

    fn refresh_meals(&mut self) {
        self.meals = self.data.meal.list_or_create_default(self.day).unwrap();
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

        Column::from(
            buttons
                .into_iter()
                .map(|(content, message)| {
                    Button::new(content)
                        .on_press(message)
                        .width(Length::Fill)
                        .into()
                })
                .collect(),
        )
        .width(200)
        .spacing(10)
        .into()
    }
}
