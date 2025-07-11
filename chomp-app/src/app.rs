use chomp_services::{Connection, Services};
use chrono::{Local, Months, NaiveDate};
use iced::{
    keyboard::{self, Modifiers},
    widget, Element, Subscription, Task,
};

use crate::widget::{
    CalorieTargetList, CalorieTargetListMessage, CreateCalorieTarget, CreateCalorieTargetMessage,
    CreateProduct, CreateProductMessage, CreateWeight, CreateWeightMessage, Dashboard,
    DashboardMessage, MealList, MealListMessage, ProductList, ProductListMessage, Tools,
    ToolsMessage, UpdateCalorieTarget, UpdateCalorieTargetMessage, UpdateProduct,
    UpdateProductMessage, UpdateWeight, UpdateWeightMessage, WeightList, WeightListMessage, Widget,
};

#[derive(Debug, Clone)]
pub enum NextWidget {
    Dashboard,
    ProductList,
    CreateProduct,
    UpdateProduct(usize),
    WeightList,
    CreateWeight,
    UpdateWeight(NaiveDate),
    MealList,
    CalorieTargetList,
    CreateCalorieTarget,
    UpdateCalorieTarget(NaiveDate),
    Tools,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabClicked,
    ShiftTabClicked,
    EscapeClicked,
    ChangeWidget(NextWidget),
    Dashboard(DashboardMessage),
    ProductList(ProductListMessage),
    CreateProduct(CreateProductMessage),
    UpdateProduct(UpdateProductMessage),
    WeightList(WeightListMessage),
    CreateWeight(CreateWeightMessage),
    UpdateWeight(UpdateWeightMessage),
    MealList(MealListMessage),
    CalorieTargetList(CalorieTargetListMessage),
    CreateCalorieTarget(CreateCalorieTargetMessage),
    UpdateCalorieTarget(UpdateCalorieTargetMessage),
    Tools(ToolsMessage),
}

pub struct Context {
    pub services: Services,
    pub next_widget: Option<NextWidget>,
}

pub struct App {
    ctx: Context,
    active_widget: Box<dyn Widget>,
}

impl App {
    pub fn new(db: Connection) -> Self {
        let services = Services::new(db);

        let weights_end = Local::now().date_naive();
        let weights_start = Local::now()
            .checked_sub_months(Months::new(1))
            .unwrap()
            .date_naive();
        let weights = services
            .weight
            .list_between(weights_start, weights_end)
            .unwrap_or_default();

        App {
            ctx: Context {
                services,
                next_widget: None,
            },
            active_widget: Box::new(Dashboard::new(weights)),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.active_widget.view()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        let widget_task = self.active_widget.update(&mut self.ctx, msg.clone());

        if let Message::ChangeWidget(w) = msg.clone() {
            self.ctx.next_widget = Some(w);
        }

        if let Some(w) = self.ctx.next_widget.take() {
            self.active_widget = match w {
                NextWidget::Dashboard => {
                    let end = Local::now().date_naive();
                    let start = Local::now()
                        .checked_sub_months(Months::new(1))
                        .unwrap()
                        .date_naive();
                    let weights = self
                        .ctx
                        .services
                        .weight
                        .list_between(start, end)
                        .unwrap_or_default();
                    Box::new(Dashboard::new(weights))
                }
                NextWidget::ProductList => {
                    let products = match self.ctx.services.product.list() {
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
                    let product = match self.ctx.services.product.read(id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateProduct::new(product))
                }
                NextWidget::WeightList => {
                    let weights = self.ctx.services.weight.list().unwrap_or_default();
                    Box::new(WeightList::new(weights))
                }
                NextWidget::CreateWeight => Box::new(CreateWeight::new()),
                NextWidget::UpdateWeight(day) => {
                    let weight = match self.ctx.services.weight.read(day) {
                        Ok(w) => w,
                        Err(err) => {
                            tracing::error!("Failed to read weight: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateWeight::new(weight))
                }
                NextWidget::MealList => {
                    let day = Local::now().date_naive();
                    let meals = match self.ctx.services.meal.list_or_create_default(day) {
                        Ok(m) => m,
                        Err(err) => {
                            tracing::error!("Failed to get meals or create default: {}", err);
                            panic!()
                        }
                    };
                    let stats = match self.ctx.services.meal.day_stats(day) {
                        Ok(s) => s,
                        Err(err) => {
                            tracing::error!("Failed to get meal stats: {}", err);
                            panic!()
                        }
                    };
                    let target = match self
                        .ctx
                        .services
                        .calorie_target
                        .read_last_or_create_default()
                    {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed to get calorie target: {}", err);
                            panic!()
                        }
                    };
                    Box::new(MealList::new(day, meals, stats, target))
                }
                NextWidget::CalorieTargetList => {
                    let targets = self.ctx.services.calorie_target.list().unwrap_or_default();
                    Box::new(CalorieTargetList::new(targets))
                }
                NextWidget::CreateCalorieTarget => Box::new(CreateCalorieTarget::new()),
                NextWidget::UpdateCalorieTarget(day) => {
                    let target = match self.ctx.services.calorie_target.read(day) {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed to read calorie target: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateCalorieTarget::new(target))
                }
                NextWidget::Tools => Box::new(Tools::new()),
            };
        }

        match msg {
            Message::TabClicked => widget::focus_next(),
            Message::ShiftTabClicked => widget::focus_previous(),
            _ => widget_task,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (keyboard::key::Named::Tab, Modifiers::SHIFT) => Some(Message::ShiftTabClicked),
                (keyboard::key::Named::Tab, _) => Some(Message::TabClicked),
                (keyboard::key::Named::Escape, _) => Some(Message::EscapeClicked),
                _ => None,
            }
        })
    }
}
