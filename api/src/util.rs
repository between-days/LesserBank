use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;

pub fn get_addr() -> (String, u16) {
    let default_host = "localhost";
    let default_port = 8080;

    let host = match env::var("HOST") {
        Ok(v) => v,
        Err(_) => default_host.to_string(),
    };

    let port = match env::var("PORT") {
        Ok(v) => v.parse().unwrap(),
        Err(_) => default_port,
    };

    println!("gets (host, port) = ({}, {})", host, port);

    (host, port)
}

pub fn get_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@localhost/postgres";
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => DEFAULT_DATABASE_URL.to_string(),
    };

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
