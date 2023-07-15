use actix_web::{dev::Server, get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::accounts::configure_accounts_api;
use repository::accounts_repository::AccountsRepoImpl;

mod api;
mod error;
mod models;
mod repository;
mod schema;
mod traits;
mod util;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn create_server() -> Result<Server, std::io::Error> {
    let pool = util::get_db_pool();

    let accounts_repo = AccountsRepoImpl::new(pool);
    let accounts_repo_data = Data::new(accounts_repo.clone());

    let s = HttpServer::new(move || {
        App::new()
            .app_data(accounts_repo_data.clone())
            .configure(configure_accounts_api::<AccountsRepoImpl>)
            .service(hello)
    })
    .bind(util::get_addr())?
    .run();

    Ok(s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let server = create_server();
    server.unwrap().await
}
