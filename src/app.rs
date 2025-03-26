use chrono::{Local, NaiveDate};
use iced::{
    keyboard::{self, Modifiers},
    widget, Element, Subscription, Task,
};
use rusqlite::Connection;

use crate::{
    data::Data,
    widget::{
        CalorieTargetList, CalorieTargetListMessage, CreateCalorieTarget,
        CreateCalorieTargetMessage, CreateProduct, CreateProductMessage, Dashboard,
        DashboardMessage, MealList, MealListMessage, ProductList, ProductListMessage,
        UpdateCalorieTarget, UpdateCalorieTargetMessage, UpdateProduct, UpdateProductMessage,
        Widget,
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
    CreateCalorieTarget,
    UpdateCalorieTarget(NaiveDate),
}

#[derive(Debug, Clone)]
pub enum Message {
    FocusNext,
    FocusPrev,
    ChangeWidget(NextWidget),
    Dashboard(DashboardMessage),
    ProductList(ProductListMessage),
    CreateProduct(CreateProductMessage),
    UpdateProduct(UpdateProductMessage),
    MealList(MealListMessage),
    CalorieTargetList(CalorieTargetListMessage),
    CreateCalorieTarget(CreateCalorieTargetMessage),
    UpdateCalorieTarget(UpdateCalorieTargetMessage),
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

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        self.active_widget.update(&mut self.ctx, msg.clone());

        if let Message::ChangeWidget(w) = msg.clone() {
            self.ctx.next_widget = Some(w);
        }

        if let Some(w) = self.ctx.next_widget.take() {
            self.active_widget = match w {
                NextWidget::Dashboard => Box::new(Dashboard::new()),
                NextWidget::ProductList => {
                    let products = match self.ctx.data.product.list() {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product list: {}", err);
                            panic!()
                        }
                    };
                    Box::new(ProductList::new(products))
                }
                NextWidget::CreateProduct => Box::new(CreateProduct::new()),
                NextWidget::UpdateProduct(id) => {
                    let product = match self.ctx.data.product.read(id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateProduct::new(product))
                }
                NextWidget::MealList => {
                    let day = Local::now().date_naive();
                    let meals = match self.ctx.data.meal.list_or_create_default(day) {
                        Ok(m) => m,
                        Err(err) => {
                            tracing::error!("Failed to get meals or create default: {}", err);
                            panic!()
                        }
                    };
                    let stats = match self.ctx.data.meal.day_stats(day) {
                        Ok(s) => s,
                        Err(err) => {
                            tracing::error!("Failed to get meal stats: {}", err);
                            panic!()
                        }
                    };
                    let target = match self.ctx.data.calorie_target.read_last_or_create_default() {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed to get calorie target: {}", err);
                            panic!()
                        }
                    };
                    Box::new(MealList::new(day, meals, stats, target))
                }
                NextWidget::CalorieTargetList => {
                    let targets = self.ctx.data.calorie_target.list().unwrap_or_default();
                    Box::new(CalorieTargetList::new(targets))
                }
                NextWidget::CreateCalorieTarget => Box::new(CreateCalorieTarget::new()),
                NextWidget::UpdateCalorieTarget(day) => {
                    let target = match self.ctx.data.calorie_target.read(day) {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed toread calorie target: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateCalorieTarget::new(target))
                }
            };
        }

        match msg {
            Message::FocusNext => widget::focus_next(),
            Message::FocusPrev => widget::focus_previous(),
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (keyboard::key::Named::Tab, Modifiers::SHIFT) => Some(Message::FocusPrev),
                (keyboard::key::Named::Tab, _) => Some(Message::FocusNext),
                _ => None,
            }
        })
    }
}
