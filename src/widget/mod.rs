use iced::Element;

use crate::app::{Context, Message};

mod create_product;
mod dashboard;
mod form_field;
mod meal_list;
mod modal;
mod product_list;
mod sidebar;
mod update_product;

pub use create_product::{CreateProduct, CreateProductMessage};
pub use dashboard::{Dashboard, DashboardMessage};
pub use meal_list::{MealList, MealListMessage};
pub use product_list::{ProductList, ProductListMessage};
pub use update_product::{UpdateProduct, UpdateProductMessage};

pub trait Widget {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, ctx: &mut Context, msg: Message);
}
