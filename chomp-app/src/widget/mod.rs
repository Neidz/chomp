use iced::{Element, Task};

use crate::app::{Context, Message};

mod create_nutrition_target;
mod create_product;
mod create_product_portion;
mod create_weight;
mod dashboard;
mod date_picker;
mod form_field;
mod line_chart;
mod meal_list;
mod modal;
mod nutrition_target_list;
mod product_list;
mod product_portion_list;
mod sidebar;
mod style;
mod tools;
mod update_nutrition_target;
mod update_product;
mod update_product_portion;
mod update_weight;
mod weight_list;

pub use create_nutrition_target::*;
pub use create_product::*;
pub use create_product_portion::*;
pub use create_weight::*;
pub use dashboard::*;
pub use date_picker::CalendarMonth;
use date_picker::*;
use form_field::*;
use line_chart::*;
pub use meal_list::*;
use modal::*;
pub use nutrition_target_list::*;
pub use product_list::*;
pub use product_portion_list::*;
use sidebar::*;
pub use tools::*;
pub use update_nutrition_target::*;
pub use update_product::*;
pub use update_product_portion::*;
pub use update_weight::*;
pub use weight_list::*;

pub trait Widget {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, ctx: &mut Context, msg: Message) -> Task<Message>;
}
