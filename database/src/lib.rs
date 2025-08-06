pub mod errors;
pub mod models;
pub mod queries;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

/* pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
} */

pub fn establish_connection() -> SqliteConnection {
    println!("🚀 PROD: Using DATABASE_URL");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_test_connection() -> SqliteConnection {
    println!("🧪 TEST: Using in-memory database");
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

    let mut connection =
        SqliteConnection::establish(":memory:").expect("Failed to create in-memory database");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    connection
}
