use super::migrate::Migration;

pub const CREATE_PRODUCTS_TABLE_QUERY_1: Migration = Migration {
    query: "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT UNIQUE NOT NULL,
            company TEXT,
            calories REAL NOT NULL,
            fats REAL NOT NULL,
            proteins REAL NOT NULL,
            carbohydrates REAL NOT NULL
        );",
    id: 1,
};

pub const CREATE_MEALS_TABLE_QUERY_2: Migration = Migration {
    query: "
        CREATE TABLE IF NOT EXISTS meals (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            position INTEGER NOT NULL,
            day TEXT NOT NULL,
            name TEXT NOT NULL,
            UNIQUE (day, name)
        );",
    id: 2,
};

pub const CREATE_MEAL_PRODUCTS_TABLE_QUERY_3: Migration = Migration {
    query: "
        CREATE TABLE IF NOT EXISTS meal_products (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            meal_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            weight REAL NOT NULL,
            UNIQUE (meal_id, product_id),
            FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
        );",
    id: 3,
};

pub const CREATE_CALORIE_TARGETS_TABLE_QUERY_4: Migration = Migration {
    query: "
        CREATE TABLE IF NOT EXISTS calorie_targets (
            day TEXT PRIMARY KEY NOT NULL,
            calories REAL NOT NULL,
            fats REAL NOT NULL,
            proteins REAL NOT NULL,
            carbohydrates REAL NOT NULL
        );",
    id: 4,
};

pub const CREATE_WEIGHTRS_TABLE_QUERY_5: Migration = Migration {
    query: "
        CREATE TABLE IF NOT EXISTS weights (
            day TEXT PRIMARY KEY NOT NULL,
            weight REAL NOT NULL
        );",
    id: 5,
};
