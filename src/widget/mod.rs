use iced::{Element, Task};

use crate::app::{Context, Message};

mod calorie_target_list;
mod create_calorie_target;
mod create_product;
mod create_weight;
mod dashboard;
mod form_field;
mod line_chart;
mod meal_list;
mod modal;
mod product_list;
mod sidebar;
mod style;
mod tools;
mod update_calorie_target;
mod update_product;
mod update_weight;
mod weight_list;

pub use calorie_target_list::*;
pub use create_calorie_target::*;
pub use create_product::*;
pub use create_weight::*;
pub use dashboard::*;
use form_field::*;
use line_chart::*;
pub use meal_list::*;
pub use product_list::*;
pub use tools::*;
pub use update_calorie_target::*;
pub use update_product::*;
pub use update_weight::*;
pub use weight_list::*;

pub trait Widget {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message>;
}
