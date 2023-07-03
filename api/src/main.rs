use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod accounts_service;
mod handlers;

fn get_addr() -> (String, u16) {
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                // Create customer account
                "/customers/{cid}/account",
                web::get().to(handlers::create_account),
            )
            .route(
                // Get customer bank account list
                "/customers/{cid}/accounts",
                web::get().to(handlers::get_accounts),
            )
            .route(
                // Get customer bank account
                "/customers/{cid}/accounts/{aid}",
                web::get().to(handlers::get_account),
            )
            .route(
                // Delete customer bank account
                "/customers/{cid}/accounts/{aid}",
                web::delete().to(handlers::delete_account),
            )
            .service(hello)
    })
    .bind(get_addr())?
    .run()
    .await
}
