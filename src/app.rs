use chrono::{Days, Local, NaiveDate};
use iced::Element;
use rusqlite::Connection;

use crate::{
    create_product_screen::{
        render_create_product_screen, CreateProductForm, CreateProductMessage,
    },
    dashboard_screen::render_dashboard_screen,
    data::{Data, DataError, Meal, MealDayStats, Product},
    form_field::InputFormFieldError,
    meal_list_screen::{render_meal_list_screen, MealListMessage},
    meal_product_form::{CopyMealProductsForm, MealProductForm, UpdateMealProductForm},
    product_list_screen::{render_product_list_screen, ProductListMessage},
    update_product_screen::{
        render_update_product_screen, UpdateProductForm, UpdateProductMessage,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Dashboard,
    CreateProduct,
    UpdateProduct(usize),
    ProductList,
    MealList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),

    NextDay,
    PrevDay,

    CreateProduct(CreateProductMessage),
    UpdateProduct(UpdateProductMessage),
    ProductList(ProductListMessage),
    MealList(MealListMessage),
}

pub struct App {
    pub data: Data,
    pub day: NaiveDate,
    pub screen: Screen,

    pub products: Vec<Product>,
    pub meals: Vec<Meal>,
    pub meal_day_stats: MealDayStats,

    pub create_product_form: Option<CreateProductForm>,
    pub update_product_form: Option<UpdateProductForm>,
    pub add_meal_product_form: Option<MealProductForm>,
    pub update_meal_product_form: Option<UpdateMealProductForm>,
    pub copy_meal_products_form: Option<CopyMealProductsForm>,
}

impl App {
    pub fn new(db: Connection) -> Self {
        let day = Local::now().date_naive();
        let data = Data::new(db);
        let products = data.product.list().unwrap();
        let meals = data.meal.list_or_create_default(day).unwrap();
        let meal_day_stats = data.meal.day_stats(day).unwrap();

        App {
            data,
            screen: Screen::Dashboard,
            day,
            products: products.clone(),
            meals,
            meal_day_stats,
            create_product_form: None,
            update_product_form: None,
            add_meal_product_form: None,
            update_meal_product_form: None,
            copy_meal_products_form: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Dashboard => render_dashboard_screen(self),
            Screen::CreateProduct => render_create_product_screen(self),
            Screen::ProductList => render_product_list_screen(self),
            Screen::MealList => render_meal_list_screen(self),
            Screen::UpdateProduct(_) => render_update_product_screen(self),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeScreen(s) => match s {
                Screen::UpdateProduct(product_id) => {
                    let p = self.data.product.read(product_id).unwrap();
                    let form = UpdateProductForm::new(&p);

                    self.update_product_form = Some(form);
                    self.screen = Screen::UpdateProduct(product_id);
                }
                Screen::CreateProduct => {
                    let form = CreateProductForm::new();
                    self.create_product_form = Some(form);
                    self.screen = Screen::CreateProduct
                }
                s => self.screen = s,
            },

            Message::PrevDay => {
                self.day = self.day.checked_sub_days(Days::new(1)).unwrap();
                self.refresh_meals();
            }
            Message::NextDay => {
                self.day = self.day.checked_add_days(Days::new(1)).unwrap();
                self.refresh_meals();
            }
            Message::CreateProduct(m) => self.handle_create_product_message(m),
            Message::UpdateProduct(m) => self.handle_update_product_message(m),
            Message::ProductList(m) => self.handle_product_list_message(m),
            Message::MealList(m) => self.handle_meal_list_message(m),
        }
    }

    fn handle_create_product_message(&mut self, message: CreateProductMessage) {
        match message {
            CreateProductMessage::UpdateFormName(name) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.name.raw_input = name;
            }
            CreateProductMessage::UpdateFormCompany(company) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.company.raw_input = company;
            }
            CreateProductMessage::UpdateFormCalories(raw_calories) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.calories.raw_input = raw_calories;
            }
            CreateProductMessage::UpdateFormFats(raw_fats) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.fats.raw_input = raw_fats;
            }
            CreateProductMessage::UpdateFormProteins(raw_proteins) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.proteins.raw_input = raw_proteins;
            }
            CreateProductMessage::UpdateFormCarbohydrates(raw_carbohydrates) => {
                let form = self.create_product_form.as_mut().unwrap();
                form.carbohydrates.raw_input = raw_carbohydrates;
            }
            CreateProductMessage::SubmitForm => {
                let form = self.create_product_form.as_mut().unwrap();
                if let Ok(product) = form.parse() {
                    if let Some(err) = self.data.product.create(product).err() {
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
                        self.create_product_form = None;
                        self.refresh_products();
                        self.screen = Screen::ProductList
                    }
                };
            }
        }
    }

    fn handle_update_product_message(&mut self, message: UpdateProductMessage) {
        match message {
            UpdateProductMessage::UpdateFormName(name) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.name.raw_input = name;
            }
            UpdateProductMessage::UpdateFormCompany(company) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.company.raw_input = company;
            }
            UpdateProductMessage::UpdateFormCalories(raw_calories) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.calories.raw_input = raw_calories;
            }
            UpdateProductMessage::UpdateFormFats(raw_fats) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.fats.raw_input = raw_fats;
            }
            UpdateProductMessage::UpdateFormProteins(raw_proteins) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.proteins.raw_input = raw_proteins;
            }
            UpdateProductMessage::UpdateFormCarbohydrates(raw_carbohydrates) => {
                let form = self.update_product_form.as_mut().unwrap();
                form.carbohydrates.raw_input = raw_carbohydrates;
            }
            UpdateProductMessage::SubmitForm => {
                let form = self.update_product_form.as_mut().unwrap();
                if let Ok(product) = form.parse() {
                    if let Some(err) = self.data.product.update(form.product_id, product).err() {
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
                        self.update_product_form = None;
                        self.refresh_products();
                        self.screen = Screen::ProductList
                    }
                };
            }
        }
    }

    fn handle_product_list_message(&mut self, message: ProductListMessage) {
        match message {
            ProductListMessage::DeleteProduct(product_id) => {
                self.data.product.delete(product_id).unwrap();
                self.refresh_products();
            }
        }
    }

    fn handle_meal_list_message(&mut self, message: MealListMessage) {
        match message {
            MealListMessage::CreateMealProductFormMeal(meal_id) => match meal_id {
                Some(id) => {
                    let meal = self.data.meal.read(id).unwrap();
                    self.add_meal_product_form = Some(MealProductForm::new(&self.products, &meal));
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
                if let Ok(add_meal_product) = self.add_meal_product_form.as_mut().unwrap().parse() {
                    self.data.meal.add_product(add_meal_product).unwrap();

                    self.refresh_meals();
                    self.add_meal_product_form = None;
                }
            }
            MealListMessage::UpdateMealProductFormMealProduct(meal_product_id) => {
                match meal_product_id {
                    Some(id) => {
                        let meal_product = self.data.meal.read_product(id).unwrap();
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
            MealListMessage::DeleteMealProduct(meal_product_id) => {
                self.data.meal.delete_product(meal_product_id).unwrap();
                self.refresh_meals();
            }
            MealListMessage::CopyMealProductsMeal(meal_id) => match meal_id {
                Some(id) => {
                    let meal = self.data.meal.read(id).unwrap();
                    let prev_day = meal.day.checked_sub_days(Days::new(1)).unwrap();
                    let prev_day_meal = self.data.meal.read_by_day_and_name(prev_day, &meal.name);

                    let meal_products = prev_day_meal.map(|m| m.products).unwrap_or_default();

                    self.copy_meal_products_form =
                        Some(CopyMealProductsForm::new(&meal_products, &prev_day, &meal));
                }
                None => self.copy_meal_products_form = None,
            },
            MealListMessage::CopyMealProductsFromDay(new_day) => {
                let form = self.copy_meal_products_form.as_mut().unwrap();

                let new_products = self
                    .data
                    .meal
                    .read_by_day_and_name(new_day, &form.target_meal.name)
                    .map(|m| m.products)
                    .unwrap_or_default();

                form.from_day = new_day;
                form.meal_products = new_products;
            }
            MealListMessage::SubmitCopyMealProductsForm => {
                if let Ok(add_meal_products) =
                    self.copy_meal_products_form.as_mut().unwrap().parse()
                {
                    add_meal_products.into_iter().for_each(|add_meal_product| {
                        let _ = self.data.meal.add_product(add_meal_product);
                    });
                    self.refresh_meals();
                    self.copy_meal_products_form = None;
                }
            }
        }
    }

    fn refresh_products(&mut self) {
        let products = self.data.product.list().unwrap();
        self.products = products.clone();
    }

    fn refresh_meals(&mut self) {
        self.meals = self.data.meal.list_or_create_default(self.day).unwrap();
        self.meal_day_stats = self.data.meal.day_stats(self.day).unwrap();
    }
}
