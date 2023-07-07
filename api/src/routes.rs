use actix_web::web;

use crate::api::accounts;

pub fn configure_accounts_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers/{customer_id}/accounts")
            .service(
                web::resource("")
                    .route(web::post().to(accounts::handlers::create_account))
                    .route(web::get().to(accounts::handlers::get_accounts)),
            )
            .service(
                web::resource("/{account_id}")
                    .route(web::get().to(accounts::handlers::get_account))
                    .route(web::delete().to(accounts::handlers::delete_account)),
            ),
    );
}
