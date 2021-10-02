pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;

use std::env;

// Import from external crates
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    // Feeds process environment from file '$PWD/.env'
    dotenv().ok();

    // Actually reads the env variable value
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // "Connect" to the sqlite database
    SqliteConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

