use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use api::accounts::routes::configure_accounts_api;
use repository::accounts::accounts_repo::AccountsRepoImpl;

mod api;
mod models;
mod repository;
mod schema;
mod traits;
mod util;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = util::get_db_pool();
    let accounts_repo = AccountsRepoImpl {};

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(accounts_repo.clone()))
            .configure(configure_accounts_api)
            .service(hello)
    })
    .bind(util::get_addr())?
    .run()
    .await
}
