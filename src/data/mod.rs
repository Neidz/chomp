use std::{cell::RefCell, rc::Rc};

use calorie_target::CalorieTargetData;
use meals::MealData;
use product::ProductData;
use rusqlite::Connection;

mod calorie_target;
mod error;
mod meals;
mod product;

pub use calorie_target::CalorieTarget;
pub use error::DataError;
pub use meals::{AddMealProduct, Meal, MealDayStats, MealProduct, UpdateMealProductWeight};
pub use product::{CreateUpdateProduct, Product};

pub struct Data {
    pub product: ProductData,
    pub meal: MealData,
    pub calorie_target: CalorieTargetData,
}

impl Data {
    pub fn new(db: Connection) -> Self {
        let db_rc = Rc::new(RefCell::new(db));

        let product = ProductData::new(db_rc.clone());
        let meal = MealData::new(db_rc.clone());
        let calorie_target = CalorieTargetData::new(db_rc.clone());

        Data {
            product,
            meal,
            calorie_target,
        }
    }
}
