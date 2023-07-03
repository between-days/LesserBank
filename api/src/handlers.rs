use actix_web::{web, Responder, Result};
use rand::Rng;
use serde::Serialize;

use crate::accounts_service::{_create_account, _get_accounts};

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

pub async fn create_account(path: web::Path<i32>) -> Result<impl Responder> {
    let customer_id = path.into_inner();
    let account_id = rand::thread_rng().gen_range(0..100);

    println!(
        "Trying to create account {} for customer {}",
        account_id, customer_id
    );

    let acc = _create_account(customer_id);

    Ok(web::Json(AccountRest {
        id: acc.id,
        customer_id: acc.customer_id,
        balance: acc.balance,
    }))
}

pub async fn get_accounts(path: web::Path<u32>) -> Result<impl Responder> {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let accs = _get_accounts();

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
