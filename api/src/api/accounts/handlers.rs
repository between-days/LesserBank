use actix_web::{web, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2::Pool, PgConnection};
use rand::Rng;
// use serde::Serialize;

#[path = "./../../repository/accounts_repo.rs"]
mod accounts_repo;

use crate::api::accounts::models::{AccountRest, AccountsRest};

pub async fn create_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
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
        accounts_repo::create_account(&mut conn, customer_id)
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
    path: web::Path<i32>,
) -> HttpResponse {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo::get_accounts(&mut conn, customer_id)
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
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    );

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo::get_account(&mut conn, customer_id, account_id)
    })
    .await;

    println!(
        "Trying to select account {}, for customer {}",
        account_id, customer_id
    );

    match res {
        Ok(account) => HttpResponse::Ok().json(web::Json(AccountRest {
            id: account.id,
            customer_id: account.customer_id,
            balance: account.balance,
        })),

        Err(_) => HttpResponse::InternalServerError()
            .body("Could not get account due to internal server error"),
    }
}

pub async fn delete_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    );

    let res = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo::delete_account(&mut conn, customer_id, account_id)
    })
    .await;

    match res {
        Ok(_) => HttpResponse::NoContent().body("Successfully deleted account"),

        Err(_) => HttpResponse::InternalServerError()
            .body("Delete account failed due to internal server error"),
    }
}
