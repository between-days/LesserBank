use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod api;
mod routes;
mod util;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = util::get_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_accounts_api)
            .service(hello)
    })
    .bind(util::get_addr())?
    .run()
    .await
}
