pub mod handlers;
pub mod models;
pub mod transform;

use actix_web::web;

use crate::{
    api::transactions,
    models::{
        account::{Account, FindAccountQuery},
        transaction::{FindTransactionQuery, NewTransaction, Transaction},
    },
    traits::{RepoCreate, RepoFind},
};

pub fn configure_transactions_api<AR, TR>(cfg: &mut web::ServiceConfig)
where
    AR: RepoFind<Account, FindAccountQuery>,
    TR: RepoCreate<Transaction, NewTransaction> + RepoFind<Transaction, FindTransactionQuery>,
{
    cfg.service(
        web::scope("/api/customers/{customer_id}/transactions").service(
            web::resource("")
                .route(web::post().to(transactions::handlers::new_internal_transaction::<AR, TR>))
                .route(web::get().to(transactions::handlers::find_transactions::<TR>)),
        ),
    );
}
