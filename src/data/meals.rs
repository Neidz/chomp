use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use chrono::NaiveDate;
use rusqlite::{params, Connection};

use super::DataError;

const DEFAULT_MEALS: [&str; 4] = ["Breakfast", "Snack", "Lunch", "Dinner"];

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MealProduct {
    pub id: usize,
    pub weight: f64,
    pub name: String,
    pub company: Option<String>,
    pub calories: f64,
    pub fats: f64,
    pub proteins: f64,
    pub carbohydrates: f64,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Meal {
    pub id: usize,
    pub day: NaiveDate,
    pub position: usize,
    pub name: String,
    pub products: Vec<MealProduct>,
}

impl PartialOrd for Meal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.position.cmp(&other.position))
    }
}

impl Ord for Meal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialEq for Meal {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Meal {}

#[derive(Debug)]
pub struct AddProductToMeal {
    pub product_id: usize,
    pub meal_id: usize,
}

#[derive(Debug)]
pub struct UpdateMealProductWeight {
    pub meal_product_id: usize,
    pub weight: f64,
}

pub struct MealData {
    db: Rc<RefCell<Connection>>,
}

#[derive(Debug)]
pub struct CreateMeal {
    pub day: NaiveDate,
    pub position: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct DayStats {
    pub calories: f64,
    pub proteins: f64,
    pub fats: f64,
    pub carbohydrates: f64,
}

#[allow(unused)]
impl MealData {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        MealData { db }
    }

    pub fn create(&self, meal: CreateMeal) -> Result<(), DataError> {
        let query = "
            INSERT INTO meals (day, name, position)
            VALUES (?1, ?2, ?3)";
        let args = params![
            format!("{}", meal.day.format("%Y-%m-%d")),
            meal.name,
            meal.position,
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn delete_product(&self, meal_product_id: usize) -> Result<(), DataError> {
        let query = "
            DELETE FROM meal_products
            WHERE id = ?1";
        let args = params![meal_product_id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn add_product(&self, add_product_to_meal: AddProductToMeal) -> Result<(), DataError> {
        let query = "
            INSERT INTO meal_products (meal_id, product_id, weight)
            VALUES (?1, ?2, ?3)";
        let args = params![
            add_product_to_meal.meal_id,
            add_product_to_meal.product_id,
            0.0
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn update_product_weight(
        &self,
        update_product_weight: UpdateMealProductWeight,
    ) -> Result<(), DataError> {
        let query = "
            UPDATE meal_products
            SET weight = ?1
            WHERE id = ?2";
        let args = params![
            update_product_weight.weight,
            update_product_weight.meal_product_id,
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn list(&self, day: NaiveDate) -> Result<Vec<Meal>, DataError> {
        let query = "
            SELECT
    			meals.id,
    			meals.day,
    			meals.name,
                meals.position,
    			meal_products.id,
    			meal_products.weight,
    			products.name,
    			products.company,
    			products.calories * meal_products.weight / 100,
    			products.fats * meal_products.weight / 100,
    			products.proteins * meal_products.weight / 100,
    			products.carbohydrates * meal_products.weight / 100
    		FROM meals
    		LEFT JOIN meal_products ON meals.id = meal_products.meal_id
    		LEFT JOIN products ON meal_products.product_id = products.id
    		WHERE meals.day = ?1";
        let args = params![format!("{}", day.format("%Y-%m-%d"))];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let rows: Vec<(usize, NaiveDate, String, usize, Option<MealProduct>)> = stmt
            .query_map(args, |row| {
                let meal_id: usize = row.get(0)?;
                let meal_day: NaiveDate = row.get(1)?;
                let meal_name: String = row.get(2)?;
                let meal_position: usize = row.get(3)?;

                let has_meal_product = row.get::<_, Option<usize>>(4)?.is_some();

                let meal_product = if has_meal_product {
                    Some(MealProduct {
                        id: row.get(4)?,
                        weight: row.get(5)?,
                        name: row.get(6)?,
                        company: row.get(7)?,
                        calories: row.get(8)?,
                        fats: row.get(9)?,
                        proteins: row.get(10)?,
                        carbohydrates: row.get(11)?,
                    })
                } else {
                    None
                };

                Ok((meal_id, meal_day, meal_name, meal_position, meal_product))
            })
            .map_err(DataError::from)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut meals: HashMap<usize, Meal> = HashMap::new();

        for (meal_id, meal_day, meal_name, meal_position, meal_product) in rows {
            let meal = meals.entry(meal_id).or_insert(Meal {
                id: meal_id,
                day: meal_day,
                name: meal_name,
                position: meal_position,
                products: Vec::new(),
            });

            if let Some(product) = meal_product {
                meal.products.push(product);
            }
        }

        let mut sorted_meals: Vec<Meal> = meals.into_values().collect();
        sorted_meals.sort();

        Ok(sorted_meals)
    }

    pub fn day_stats(&self, day: NaiveDate) -> Result<DayStats, DataError> {
        let query = "
            SELECT
                COALESCE(SUM(products.calories * meal_products.weight / 100), 0) AS total_calories,
                COALESCE(SUM(products.fats * meal_products.weight / 100), 0) AS total_fats,
                COALESCE(SUM(products.proteins * meal_products.weight / 100), 0) AS total_proteins,
                COALESCE(SUM(products.carbohydrates * meal_products.weight / 100), 0) AS total_carbohydrates
            FROM meals
            LEFT JOIN meal_products ON meals.id = meal_products.meal_id
            LEFT JOIN products ON meal_products.product_id = products.id
            WHERE meals.day = ?1
            GROUP BY meals.day";
        let args = params![format!("{}", day.format("%Y-%m-%d"))];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row(args, |row| {
            Ok(DayStats {
                calories: row.get(0)?,
                fats: row.get(1)?,
                proteins: row.get(2)?,
                carbohydrates: row.get(3)?,
            })
        })
        .map_err(DataError::from)
    }

    pub fn list_or_create_default(&self, day: NaiveDate) -> Result<Vec<Meal>, DataError> {
        match self.list(day) {
            Ok(m) => {
                if m.is_empty() {
                    self.create_default(day)?;
                    self.list(day)
                } else {
                    Ok(m)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn create_default(&self, day: NaiveDate) -> Result<(), DataError> {
        DEFAULT_MEALS
            .into_iter()
            .enumerate()
            .try_for_each(|(i, name)| {
                let meal = CreateMeal {
                    day,
                    position: i,
                    name: name.to_owned(),
                };

                self.create(meal)
            })
    }
}
