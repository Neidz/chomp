use std::{cell::RefCell, rc::Rc};

use meals::MealData;
use product::ProductData;
use rusqlite::Connection;

mod error;
mod meals;
mod product;

pub use error::DataError;
pub use meals::{AddProductToMeal, DayStats, Meal, UpdateMealProductWeight};
pub use product::{CreateUpdateProduct, Product};

pub struct Data {
    pub product: ProductData,
    pub meal: MealData,
}

impl Data {
    pub fn new(db: Connection) -> Self {
        let db_rc = Rc::new(RefCell::new(db));

        let product = ProductData::new(db_rc.clone());
        let meal = MealData::new(db_rc.clone());

        Data { product, meal }
    }
}
