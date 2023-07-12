use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
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
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = util::get_db_pool();

    let accounts_repo = AccountsRepoImpl::new(pool);
    let accounts_repo_data = Data::new(accounts_repo.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(accounts_repo_data.clone())
            .configure(configure_accounts_api::<AccountsRepoImpl>)
            .service(hello)
    })
    .bind(util::get_addr())?
    .run()
    .await
}
