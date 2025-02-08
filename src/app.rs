use iced::{
    widget::{column, row, Button, Column, Text},
    Element, Length,
};
use rusqlite::Connection;

use crate::{
    data::{Data, DataError, Product},
    form_field::InputFormFieldError,
    product_form::{render_product_form, CreateUpdateProductForm},
    product_list::render_product_list,
};

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
}

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    CreateProduct,
    ProductList,
}

pub struct App {
    data: Data,
    products: Vec<Product>,
    screen: Screen,
    create_product_form: CreateUpdateProductForm,
}

impl App {
    pub fn new(db: Connection) -> Self {
        let data = Data::new(db);
        let products = data.product.list().unwrap();

        App {
            data,
            products,
            screen: Screen::Home,
            create_product_form: CreateUpdateProductForm::new(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = match self.screen {
            Screen::Home => self.home_screen(),
            Screen::CreateProduct => self.create_product_screen(),
            Screen::ProductList => self.product_list_screen(),
        };

        row![self.sidebar(), content].padding(20).spacing(20).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeScreen(s) => {
                self.screen = s;
            }
            Message::UpdateCreateProductFormName(s) => {
                self.create_product_form.name.raw_input = s;
            }
            Message::UpdateCreateProductFormCompany(s) => {
                self.create_product_form.company.raw_input = s;
            }
            Message::UpdateCreateProductFormCalories(s) => {
                self.create_product_form.calories.raw_input = s;
            }
            Message::UpdateCreateProductFormFats(s) => {
                self.create_product_form.fats.raw_input = s;
            }
            Message::UpdateCreateProductFormProteins(s) => {
                self.create_product_form.proteins.raw_input = s;
            }
            Message::UpdateCreateProductFormCarbohydrates(s) => {
                self.create_product_form.carbohydrates.raw_input = s;
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
                        self.screen = Screen::ProductList
                    }
                };
            }
        }
    }

    fn home_screen(&self) -> Element<Message> {
        column![Text::new("Home").size(40)].spacing(10).into()
    }

    fn create_product_screen(&self) -> Element<Message> {
        column![
            Text::new("Create Product").size(40),
            render_product_form(&self.create_product_form)
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

    fn sidebar(&self) -> Element<Message> {
        let buttons = vec![
            ("Home", Message::ChangeScreen(Screen::Home)),
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
