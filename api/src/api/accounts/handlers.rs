use actix_web::{error, web, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2::Pool, PgConnection};
use rand::Rng;

use super::models::{AccountRest, AccountsRest};
use crate::repository::accounts::accounts_repo::AccountsRepoImpl;
use crate::repository::error::RepoError;
use crate::traits::AccountsRepository;

pub async fn create_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    accounts_repo: web::Data<AccountsRepoImpl>,
    path: web::Path<i32>,
) -> HttpResponse {
    let customer_id = path.into_inner();
    let account_id = rand::thread_rng().gen_range(0..100);

    println!(
        "Trying to create account {} for customer {}",
        account_id, customer_id
    );

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo.create_account(&mut conn, customer_id)
    })
    .await;

    match res {
        Ok(account) => HttpResponse::Ok().json(web::Json(AccountRest {
            id: account.id,
            customer_id: account.customer_id,
            balance: account.balance,
        })),

        Err(_) => HttpResponse::InternalServerError()
            .body("Cannot geet Accounts for customer due to internal server error"),
    }
}

pub async fn get_accounts(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    accounts_repo: web::Data<AccountsRepoImpl>,
    path: web::Path<i32>,
) -> HttpResponse {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo.get_accounts(&mut conn, customer_id)
    })
    .await;

    match res {
        Ok(accounts) => HttpResponse::Ok().json(web::Json(AccountsRest {
            accounts: accounts
                .iter()
                .map(|acc| AccountRest {
                    id: acc.id,
                    customer_id: acc.customer_id,
                    balance: acc.balance,
                })
                .collect(),
        })),

        Err(_) => HttpResponse::InternalServerError()
            .body("Cannot geet Accounts for customer due to internal server error"),
    }
}

pub async fn get_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    accounts_repo: web::Data<AccountsRepoImpl>,
    path: web::Path<(i32, i32)>,
) -> Result<actix_web::web::Json<AccountRest>, actix_web::Error> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    );

    let account = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo.get_account(&mut conn, customer_id, account_id)
    })
    .await
    .map_err(|err| error::ErrorInternalServerError(err))?
    .map_err(|err| match err {
        RepoError::NotFound => error::ErrorNotFound(err),
        RepoError::Other => error::ErrorInternalServerError(err),
    })?;

    Ok(web::Json(AccountRest {
        id: account.id,
        customer_id: account.customer_id,
        balance: account.balance,
    }))
}

pub async fn delete_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    accounts_repo: web::Data<AccountsRepoImpl>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    );

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo.delete_account(&mut conn, customer_id, account_id)
    })
    .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body("Successfully deleted account"),

        Err(_) => HttpResponse::InternalServerError()
            .body("Delete account failed due to internal server error"),
    }
}
