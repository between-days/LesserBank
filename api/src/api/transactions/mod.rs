pub mod handlers;
pub mod models;
pub mod transform;

use actix_web::web;

use crate::{
    api::transactions,
    traits::{AccountsRepository, TransactionsRepository},
};

pub fn configure_transactions_api<AR: AccountsRepository, TR: TransactionsRepository>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/api/customers/{customer_id}/transactions").service(
            web::resource("")
                .route(web::post().to(transactions::handlers::new_internal_transaction::<AR, TR>))
                .route(web::get().to(transactions::handlers::find_transactions::<AR, TR>)),
        ),
    );
}
