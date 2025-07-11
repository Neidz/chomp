use std::{cell::RefCell, rc::Rc};

use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection};

use super::ServiceError;

#[derive(Debug, Clone)]
pub struct NutritionTarget {
    pub day: NaiveDate,
    pub calories: f32,
    pub fats: f32,
    pub proteins: f32,
    pub carbohydrates: f32,
}

impl NutritionTarget {
    pub fn new(
        day: NaiveDate,
        calories: f32,
        fats: f32,
        proteins: f32,
        carbohydrates: f32,
    ) -> Self {
        NutritionTarget {
            day,
            calories,
            fats,
            proteins,
            carbohydrates,
        }
    }
}

pub struct NutritionTargetService {
    db: Rc<RefCell<Connection>>,
}

impl NutritionTargetService {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        NutritionTargetService { db }
    }

    pub fn create(&self, target: NutritionTarget) -> Result<(), ServiceError> {
        let query = "
            INSERT INTO nutrition_targets (day, calories, fats, proteins, carbohydrates)
    	    VALUES (?1, ?2, ?3, ?4, ?5)";
        let args = params![
            format!("{}", target.day.format("%Y-%m-%d")),
            target.calories,
            target.fats,
            target.proteins,
            target.carbohydrates
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn update(&self, target: NutritionTarget) -> Result<(), ServiceError> {
        let query = "
            UPDATE nutrition_targets
            SET calories=?1, fats=?2, proteins=?3, carbohydrates=?4
            WHERE day = ?5";
        let args = params![
            target.calories,
            target.fats,
            target.proteins,
            target.carbohydrates,
            format!("{}", target.day.format("%Y-%m-%d")),
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn read(&self, day: NaiveDate) -> Result<NutritionTarget, ServiceError> {
        let query = "
            SELECT day, calories, fats, proteins, carbohydrates
            FROM nutrition_targets
    		WHERE day = ?1";
        let args = params![format!("{}", day.format("%Y-%m-%d")),];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row(args, |row| {
            Ok(NutritionTarget {
                day: row.get(0)?,
                calories: row.get(1)?,
                fats: row.get(2)?,
                proteins: row.get(3)?,
                carbohydrates: row.get(4)?,
            })
        })
        .map_err(ServiceError::from)
    }

    pub fn read_last(&self) -> Result<NutritionTarget, ServiceError> {
        let query = "
            SELECT day, calories, fats, proteins, carbohydrates
            FROM nutrition_targets
            ORDER BY day DESC
            LIMIT 1";

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row([], |row| {
            Ok(NutritionTarget {
                day: row.get(0)?,
                calories: row.get(1)?,
                fats: row.get(2)?,
                proteins: row.get(3)?,
                carbohydrates: row.get(4)?,
            })
        })
        .map_err(ServiceError::from)
    }

    pub fn read_last_or_create_default(&self) -> Result<NutritionTarget, ServiceError> {
        match self.read_last() {
            Ok(t) => Ok(t),
            Err(ServiceError::NoRows) => {
                let today = Local::now().date_naive();
                let target = NutritionTarget::new(today, 2500.0, 80.0, 200.0, 245.0);
                self.create(target)?;
                self.read_last()
            }
            err => err,
        }
    }

    pub fn delete(&self, day: NaiveDate) -> Result<(), ServiceError> {
        let query = "
            DELETE FROM nutrition_targets
    	    WHERE day = ?1";
        let args = params![format!("{}", day.format("%Y-%m-%d"))];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<NutritionTarget>, ServiceError> {
        let query = "
            SELECT day, calories, fats, proteins, carbohydrates
            FROM nutrition_targets
            ORDER BY day DESC";

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let targets = stmt
            .query_map([], |row| {
                Ok(NutritionTarget {
                    day: row.get(0)?,
                    calories: row.get(1)?,
                    fats: row.get(2)?,
                    proteins: row.get(3)?,
                    carbohydrates: row.get(4)?,
                })
            })
            .map_err(ServiceError::from)?
            .collect::<Result<Vec<NutritionTarget>, _>>()?;

        Ok(targets)
    }
}
