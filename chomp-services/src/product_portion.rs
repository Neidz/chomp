use std::{cell::RefCell, rc::Rc};

use rusqlite::{params, Connection};
use serde::Deserialize;

use super::ServiceError;

#[derive(Debug, Clone)]
pub struct ProductPortion {
    pub id: usize,
    pub name: String,
    pub product_id: usize,
    pub weight: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProductPortion {
    pub name: String,
    pub product_id: usize,
    pub weight: f32,
}

pub struct ProductPortionService {
    db: Rc<RefCell<Connection>>,
}

impl ProductPortionService {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        ProductPortionService { db }
    }

    pub fn create(&self, product_portion: CreateProductPortion) -> Result<(), ServiceError> {
        let query = "
            INSERT INTO product_portions (name, product_id, weight)
    	    VALUES (?1, ?2, ?3)";
        let args = params![
            product_portion.name,
            product_portion.product_id,
            product_portion.weight
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn update(&self, product_portion: ProductPortion) -> Result<(), ServiceError> {
        let query = "
            UPDATE product_portions
            SET name=?1, weight=?2
            WHERE id = ?3";
        let args = params![
            product_portion.name,
            product_portion.weight,
            product_portion.id,
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn read(&self, id: usize) -> Result<ProductPortion, ServiceError> {
        let query = "
            SELECT id, name, product_id, weight
            FROM product_portions
    		WHERE id = ?1";
        let args = params![id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row(args, |row| {
            Ok(ProductPortion {
                id: row.get(0)?,
                name: row.get(1)?,
                product_id: row.get(2)?,
                weight: row.get(3)?,
            })
        })
        .map_err(ServiceError::from)
    }

    pub fn delete(&self, id: usize) -> Result<(), ServiceError> {
        let query = "
            DELETE FROM product_portions
    	    WHERE id = ?1";
        let args = params![id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(ServiceError::from)?;

        Ok(())
    }

    pub fn list(&self, product_id: usize) -> Result<Vec<ProductPortion>, ServiceError> {
        let query = "
            SELECT id, name, product_id, weight
            FROM product_portions
            WHERE product_id = ?1
            ORDER BY id ASC";
        let args = params![product_id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let product_portions = stmt
            .query_map(args, |row| {
                Ok(ProductPortion {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    product_id: row.get(2)?,
                    weight: row.get(3)?,
                })
            })
            .map_err(ServiceError::from)?
            .collect::<Result<Vec<ProductPortion>, _>>()?;

        Ok(product_portions)
    }
}
