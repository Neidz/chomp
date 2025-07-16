use std::{cell::RefCell, rc::Rc};

use meals::MealService;
use nutrition_target::NutritionTargetService;
use product::ProductService;
pub use rusqlite::Connection;
use weight::WeightService;

mod error;
mod meals;
mod nutrition_target;
mod product;
mod product_portion;
mod weight;

pub use error::ServiceError;
pub use meals::{AddMealProduct, Meal, MealDayStats, MealProduct, UpdateMealProductWeight};
pub use nutrition_target::NutritionTarget;
pub use product::{CreateUpdateProduct, Product};
pub use product_portion::{CreateProductPortion, ProductPortion};
pub use weight::Weight;

use crate::product_portion::ProductPortionService;

pub struct Services {
    pub product: ProductService,
    pub product_portions: ProductPortionService,
    pub weight: WeightService,
    pub meal: MealService,
    pub nutrition_target: NutritionTargetService,
}

impl Services {
    pub fn new(db: Connection) -> Self {
        let db_rc = Rc::new(RefCell::new(db));

        let product = ProductService::new(db_rc.clone());
        let product_portions = ProductPortionService::new(db_rc.clone());
        let weight = WeightService::new(db_rc.clone());
        let meal = MealService::new(db_rc.clone());
        let nutrition_target = NutritionTargetService::new(db_rc.clone());

        Services {
            product,
            product_portions,
            weight,
            meal,
            nutrition_target,
        }
    }
}
