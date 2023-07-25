pub mod handlers;
pub mod models;
pub mod transform;
pub mod util;

use actix_web::web;

use crate::{
    api::accounts,
    models::account::{Account, FindAccountQuery, NewAccount},
    traits::{RepoCreate, RepoDeleteById, RepoFind, RepoGetById},
};

pub fn configure_accounts_api<AR>(cfg: &mut web::ServiceConfig)
where
    AR: RepoCreate<Account, NewAccount>
        + RepoFind<Account, FindAccountQuery>
        + RepoGetById<Account>
        + RepoDeleteById<Account>,
{
    cfg.service(
        web::scope("/api/customers/{customer_id}/accounts")
            .service(
                web::resource("")
                    .route(web::post().to(accounts::handlers::create_account::<AR>))
                    .route(web::get().to(accounts::handlers::find_accounts::<AR>)),
            )
            .service(
                web::resource("/{account_id}")
                    .route(web::get().to(accounts::handlers::get_account::<AR>))
                    .route(web::delete().to(accounts::handlers::delete_account::<AR>)),
            ),
    );
}
