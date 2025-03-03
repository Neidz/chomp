use chrono::Local;
use iced::Element;
use rusqlite::Connection;

use crate::{
    data::Data,
    widget::{
        CalorieTargetList, CalorieTargetListMessage, CreateProduct, CreateProductMessage,
        Dashboard, DashboardMessage, MealList, MealListMessage, ProductList, ProductListMessage,
        UpdateProduct, UpdateProductMessage, Widget,
    },
};

#[derive(Debug, Clone)]
pub enum NextWidget {
    Dashboard,
    ProductList,
    CreateProduct,
    UpdateProduct(usize),
    MealList,
    CalorieTargetList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeWidget(NextWidget),
    Dashboard(DashboardMessage),
    ProductList(ProductListMessage),
    CreateProduct(CreateProductMessage),
    UpdateProduct(UpdateProductMessage),
    MealList(MealListMessage),
    CalorieTargetList(CalorieTargetListMessage),
}

pub struct Context {
    pub data: Data,
    pub next_widget: Option<NextWidget>,
}

pub struct App {
    ctx: Context,
    active_widget: Box<dyn Widget>,
}

impl App {
    pub fn new(db: Connection) -> Self {
        let data = Data::new(db);

        App {
            ctx: Context {
                data,
                next_widget: None,
            },
            active_widget: Box::new(Dashboard::new()),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.active_widget.view()
    }

    pub fn update(&mut self, msg: Message) {
        match msg.clone() {
            Message::ChangeWidget(w) => {
                self.ctx.next_widget = Some(w);
            }
            _ => {}
        }

        if let Some(w) = self.ctx.next_widget.take() {
            self.active_widget = match w {
                NextWidget::Dashboard => Box::new(Dashboard::new()),
                NextWidget::ProductList => {
                    let products = self.ctx.data.product.list().unwrap();
                    Box::new(ProductList::new(products))
                }
                NextWidget::CreateProduct => Box::new(CreateProduct::new()),
                NextWidget::UpdateProduct(id) => {
                    let product = self.ctx.data.product.read(id).unwrap();
                    Box::new(UpdateProduct::new(product))
                }
                NextWidget::MealList => {
                    let day = Local::now().date_naive();
                    let meals = self.ctx.data.meal.list_or_create_default(day).unwrap();
                    let stats = self.ctx.data.meal.day_stats(day).unwrap();
                    let target = self
                        .ctx
                        .data
                        .calorie_target
                        .read_last_or_create_default()
                        .unwrap();
                    Box::new(MealList::new(day, meals, stats, target))
                }
                NextWidget::CalorieTargetList => {
                    let targets = self.ctx.data.calorie_target.list().unwrap_or_default();
                    Box::new(CalorieTargetList::new(targets))
                }
            };
        }

        self.active_widget.update(&mut self.ctx, msg);
    }
}
