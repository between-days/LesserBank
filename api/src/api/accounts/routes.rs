use actix_web::web;

use crate::{api::accounts, traits::AccountsRepository};

pub fn configure_accounts_api<T: AccountsRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers/{customer_id}/accounts")
            .service(
                web::resource("")
                    .route(web::post().to(accounts::handlers::create_account::<T>))
                    .route(web::get().to(accounts::handlers::get_accounts::<T>)),
            )
            .service(
                web::resource("/{account_id}")
                    .route(web::get().to(accounts::handlers::get_account::<T>))
                    .route(web::delete().to(accounts::handlers::delete_account::<T>)),
            ),
    );
}
