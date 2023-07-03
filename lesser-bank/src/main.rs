use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                //Create customer account
                "/customers/{cid}/account",
                web::post().to(handlers::create_account),
            )
            .route(
                //Get customer bank account list
                "/customers/{cid}/accounts",
                web::get().to(handlers::get_accounts),
            )
            .route(
                //Get customer bank account
                "/customers/{cid}/accounts/{aid}",
                web::get().to(handlers::get_account),
            )
            .route(
                //Delete customer bank account
                "/customers/{cid}/accounts/{aid}",
                web::delete().to(handlers::delete_account),
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
