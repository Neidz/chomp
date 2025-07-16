use chomp_services::{Connection, Services};
use chrono::{Local, Months, NaiveDate};
use iced::{
    keyboard::{self, Modifiers},
    widget, Element, Subscription, Task,
};

use crate::widget::{
    CalendarMonth, CreateNutritionTarget, CreateNutritionTargetMessage, CreateProduct,
    CreateProductMessage, CreateProductPortion, CreateProductPortionMessage, CreateWeight,
    CreateWeightMessage, Dashboard, DashboardMessage, MealList, MealListMessage,
    NutritionTargetList, NutritionTargetListMessage, ProductList, ProductListMessage,
    ProductPortionList, ProductPortionListMessage, Tools, ToolsMessage, UpdateNutritionTarget,
    UpdateNutritionTargetMessage, UpdateProduct, UpdateProductMessage, UpdateProductPortion,
    UpdateProductPortionMessage, UpdateWeight, UpdateWeightMessage, WeightList, WeightListMessage,
    Widget,
};

type ProductId = usize;
type ProductPortionId = usize;

#[derive(Debug, Clone)]
pub enum NextWidget {
    Dashboard,
    ProductList,
    CreateProduct,
    UpdateProduct(ProductId),
    ProductPortionList(ProductId),
    CreateProductPortion(ProductId),
    UpdateProductPortion(ProductId, ProductPortionId),
    WeightList,
    CreateWeight,
    UpdateWeight(NaiveDate),
    MealList,
    NutritionTargetList,
    CreateNutritionTarget,
    UpdateNutritionTarget(NaiveDate),
    Tools,
}

#[derive(Debug, Clone)]
pub enum Message {
    DatePickerDateChange(NaiveDate),
    DatePickerYearChange(i32),
    DatePickerMonthChange(CalendarMonth),
    OpenDatePicker,
    CloseDatePicker,
    TabClicked,
    ShiftTabClicked,
    EscapeClicked,
    ChangeWidget(NextWidget),
    Dashboard(DashboardMessage),
    ProductList(ProductListMessage),
    CreateProduct(CreateProductMessage),
    UpdateProduct(UpdateProductMessage),
    ProductPortionList(ProductPortionListMessage),
    CreateProductPortion(CreateProductPortionMessage),
    UpdateProductPortion(UpdateProductPortionMessage),
    WeightList(WeightListMessage),
    CreateWeight(CreateWeightMessage),
    UpdateWeight(UpdateWeightMessage),
    MealList(MealListMessage),
    NutritionTargetList(NutritionTargetListMessage),
    CreateNutritionTarget(CreateNutritionTargetMessage),
    UpdateNutritionTarget(UpdateNutritionTargetMessage),
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
                            std::process::exit(1);
                        }
                    };
                    Box::new(ProductList::new(products))
                }
                NextWidget::CreateProduct => Box::new(CreateProduct::new()),
                NextWidget::UpdateProduct(product_id) => {
                    let product = match self.ctx.services.product.read(product_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            std::process::exit(1);
                        }
                    };
                    Box::new(UpdateProduct::new(product))
                }
                NextWidget::ProductPortionList(product_id) => {
                    let product = match self.ctx.services.product.read(product_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            std::process::exit(1);
                        }
                    };
                    let portions = match self.ctx.services.product_portion.list(product_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product portion list: {}", err);
                            std::process::exit(1);
                        }
                    };
                    Box::new(ProductPortionList::new(product, portions))
                }
                NextWidget::CreateProductPortion(product_id) => {
                    let product = match self.ctx.services.product.read(product_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            std::process::exit(1);
                        }
                    };

                    Box::new(CreateProductPortion::new(&product))
                }
                NextWidget::UpdateProductPortion(product_id, product_portion_id) => {
                    let product = match self.ctx.services.product.read(product_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product by id: {}", err);
                            std::process::exit(1);
                        }
                    };
                    let portion = match self.ctx.services.product_portion.read(product_portion_id) {
                        Ok(p) => p,
                        Err(err) => {
                            tracing::error!("Failed to get product portion list: {}", err);
                            std::process::exit(1);
                        }
                    };
                    Box::new(UpdateProductPortion::new(&product, &portion))
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
                        .nutrition_target
                        .read_last_or_create_default()
                    {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed to get nutrition target: {}", err);
                            panic!()
                        }
                    };
                    let portions = match self.ctx.services.product_portion.list_all() {
                        Ok(m) => m,
                        Err(err) => {
                            tracing::error!("Failed to get product portions: {}", err);
                            panic!()
                        }
                    };

                    Box::new(MealList::new(day, meals, stats, target, portions))
                }
                NextWidget::NutritionTargetList => {
                    let targets = self
                        .ctx
                        .services
                        .nutrition_target
                        .list()
                        .unwrap_or_default();
                    Box::new(NutritionTargetList::new(targets))
                }
                NextWidget::CreateNutritionTarget => Box::new(CreateNutritionTarget::new()),
                NextWidget::UpdateNutritionTarget(day) => {
                    let target = match self.ctx.services.nutrition_target.read(day) {
                        Ok(t) => t,
                        Err(err) => {
                            tracing::error!("Failed to read nutrition target: {}", err);
                            panic!()
                        }
                    };
                    Box::new(UpdateNutritionTarget::new(target))
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
