use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@localhost/postgres";

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => DEFAULT_DATABASE_URL.to_string(),
    };

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
