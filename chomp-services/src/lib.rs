use std::{cell::RefCell, rc::Rc};

use calorie_target::CalorieTargetService;
use meals::MealService;
use product::ProductService;
pub use rusqlite::Connection;
use weight::WeightService;

mod calorie_target;
mod error;
mod meals;
mod product;
mod weight;

pub use calorie_target::CalorieTarget;
pub use error::ServiceError;
pub use meals::{AddMealProduct, Meal, MealDayStats, MealProduct, UpdateMealProductWeight};
pub use product::{CreateUpdateProduct, Product};
pub use weight::Weight;

pub struct Services {
    pub product: ProductService,
    pub weight: WeightService,
    pub meal: MealService,
    pub calorie_target: CalorieTargetService,
}

impl Services {
    pub fn new(db: Connection) -> Self {
        let db_rc = Rc::new(RefCell::new(db));

        let product = ProductService::new(db_rc.clone());
        let weight = WeightService::new(db_rc.clone());
        let meal = MealService::new(db_rc.clone());
        let calorie_target = CalorieTargetService::new(db_rc.clone());

        Services {
            product,
            weight,
            meal,
            calorie_target,
        }
    }
}
