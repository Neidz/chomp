use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDate;
use rusqlite::{params, Connection};

use super::ServiceError;

#[derive(Debug, Clone)]
pub struct Weight {
    pub day: NaiveDate,
    pub weight: f32,
}

impl Weight {
    pub fn new(day: NaiveDate, weight: f32) -> Self {
        Weight { day, weight }
    }
}

pub struct WeightService {
    db: Rc<RefCell<Connection>>,
}

impl WeightService {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        WeightService { db }
    }

    pub fn create(&self, weight: Weight) -> Result<(), ServiceError> {
        let query = "
            INSERT INTO weights (day, weight)
    	    VALUES (?1, ?2)";
        let args = params![format!("{}", weight.day.format("%Y-%m-%d")), weight.weight,];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn update(&self, weight: Weight) -> Result<(), ServiceError> {
        let query = "
            UPDATE weights
            SET weight=?1
            WHERE day = ?2";
        let args = params![weight.weight, format!("{}", weight.day.format("%Y-%m-%d")),];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn read(&self, day: NaiveDate) -> Result<Weight, ServiceError> {
        let query = "
            SELECT day, weight
            FROM weights
    		WHERE day = ?1";
        let args = params![format!("{}", day.format("%Y-%m-%d")),];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row(args, |row| {
            Ok(Weight {
                day: row.get(0)?,
                weight: row.get(1)?,
            })
        })
        .map_err(ServiceError::from)
    }

    pub fn delete(&self, day: NaiveDate) -> Result<(), ServiceError> {
        let query = "
            DELETE FROM weights
    	    WHERE day = ?1";
        let args = params![format!("{}", day.format("%Y-%m-%d"))];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Weight>, ServiceError> {
        let query = "
            SELECT day, weight
            FROM weights
            ORDER BY day DESC";

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let weights = stmt
            .query_map([], |row| {
                Ok(Weight {
                    day: row.get(0)?,
                    weight: row.get(1)?,
                })
            })
            .map_err(ServiceError::from)?
            .collect::<Result<Vec<Weight>, _>>()?;

        Ok(weights)
    }

    pub fn list_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Weight>, ServiceError> {
        let query = "
            SELECT day, weight
            FROM weights
            WHERE day between ?1 AND ?2
            ORDER BY day DESC";
        let args = params![
            format!("{}", start.format("%Y-%m-%d")),
            format!("{}", end.format("%Y-%m-%d"))
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let weights = stmt
            .query_map(args, |row| {
                Ok(Weight {
                    day: row.get(0)?,
                    weight: row.get(1)?,
                })
            })
            .map_err(ServiceError::from)?
            .collect::<Result<Vec<Weight>, _>>()?;

        Ok(weights)
    }
}
