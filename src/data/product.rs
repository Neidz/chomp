use std::{cell::RefCell, fmt, rc::Rc};

use rusqlite::{params, Connection};
use serde::Deserialize;

use super::DataError;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub company: Option<String>,
    pub calories: f32,
    pub fats: f32,
    pub proteins: f32,
    pub carbohydrates: f32,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUpdateProduct {
    pub name: String,
    pub company: Option<String>,
    pub calories: f32,
    pub fats: f32,
    pub proteins: f32,
    pub carbohydrates: f32,
}

pub struct ProductData {
    db: Rc<RefCell<Connection>>,
}

#[allow(unused)]
impl ProductData {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        ProductData { db }
    }

    pub fn create(&self, product: CreateUpdateProduct) -> Result<(), DataError> {
        let query = "
            INSERT INTO products (name, company, calories, fats, proteins, carbohydrates)
    	    VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        let args = params![
            product.name,
            product.company,
            product.calories,
            product.fats,
            product.proteins,
            product.carbohydrates
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn update(&self, id: usize, product: CreateUpdateProduct) -> Result<(), DataError> {
        let query = "
            UPDATE products
            SET name=?1, company=?2, calories=?3, fats=?4, proteins=?5, carbohydrates=?6
            WHERE id = ?7";
        let args = params![
            product.name,
            product.company,
            product.calories,
            product.fats,
            product.proteins,
            product.carbohydrates,
            id
        ];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn read(&self, id: usize) -> Result<Product, DataError> {
        let query = "
            SELECT id, name, company, calories, fats, proteins, carbohydrates
            FROM products
    		WHERE id = ?1";
        let args = params![id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        stmt.query_row(args, |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                company: row.get(2)?,
                calories: row.get(3)?,
                fats: row.get(4)?,
                proteins: row.get(5)?,
                carbohydrates: row.get(6)?,
            })
        })
        .map_err(DataError::from)
    }

    pub fn delete(&self, id: usize) -> Result<(), DataError> {
        let query = "
            DELETE FROM products
    	    WHERE id = ?1";
        let args = params![id];

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;
        stmt.execute(args).map_err(DataError::from)?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Product>, DataError> {
        let query = "
            SELECT id, name, company, calories, fats, proteins, carbohydrates
            FROM products
            ORDER BY id ASC";

        let db = self.db.borrow();
        let mut stmt = db.prepare(query)?;

        let products = stmt
            .query_map([], |row| {
                Ok(Product {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    company: row.get(2)?,
                    calories: row.get(3)?,
                    fats: row.get(4)?,
                    proteins: row.get(5)?,
                    carbohydrates: row.get(6)?,
                })
            })
            .map_err(DataError::from)?
            .collect::<Result<Vec<Product>, _>>()?;

        Ok(products)
    }
}
