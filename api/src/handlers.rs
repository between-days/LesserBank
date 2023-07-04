use actix_web::{web, Responder, Result};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2::Pool, PgConnection};
use rand::Rng;
use serde::Serialize;

#[path = "./repository/accounts_repo.rs"]
mod accounts_repo;

#[derive(Serialize)]
struct AccountRest {
    id: i32,
    customer_id: i32,
    balance: i32,
}

#[derive(Serialize)]
struct AccountsRest {
    accounts: Vec<AccountRest>,
}

pub async fn create_account(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let customer_id = path.into_inner();
    let account_id = rand::thread_rng().gen_range(0..100);

    println!(
        "Trying to create account {} for customer {}",
        account_id, customer_id
    );

    let acc = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo::create_account(&mut conn, customer_id)
    })
    .await?;
    // .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(AccountRest {
        id: acc.id,
        customer_id: acc.customer_id,
        balance: acc.balance,
    }))
}

pub async fn get_accounts(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let accs = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        accounts_repo::get_accounts(&mut conn, customer_id)
    })
    .await?;
    // .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(AccountsRest {
        accounts: accs
            .iter()
            .map(|acc| AccountRest {
                id: acc.id,
                customer_id: acc.customer_id,
                balance: acc.balance,
            })
            .collect(),
    }))
}
pub async fn get_account(path: web::Path<(u32, u32)>) -> Result<String> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    );

    Ok(format!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    ))
}

pub async fn delete_account(path: web::Path<(u32, u32)>) -> Result<String> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    );

    Ok(format!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    ))
}
