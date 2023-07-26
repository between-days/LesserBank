use actix_cors::Cors;
use actix_web::{dev::Server, get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::accounts::configure_accounts_api;
use api::transactions::configure_transactions_api;
use repository::{
    accounts_repository::AccountsRepoImpl, transactions_repository::TransactionsRepoImpl,
};

mod api;
mod error;
mod models;
mod repository;
mod traits;
mod util;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn create_server() -> Result<Server, std::io::Error> {
    let pool = util::get_db_pool();

    let pool_r = pool.clone();
    let pool_t = pool.clone();

    let accounts_repo = AccountsRepoImpl::new(pool_r);
    let transaction_repo = TransactionsRepoImpl::new(pool_t);

    let ar_data = Data::new(accounts_repo);
    let tr_data = Data::new(transaction_repo);

    let s = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(ar_data.clone())
            .app_data(tr_data.clone())
            .configure(configure_accounts_api::<AccountsRepoImpl>)
            .configure(configure_transactions_api::<AccountsRepoImpl, TransactionsRepoImpl>)
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
