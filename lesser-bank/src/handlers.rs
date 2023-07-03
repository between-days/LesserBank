use actix_web::{web, Result};

pub async fn create_account(path: web::Path<(u32, u32)>) -> Result<String> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to create account {}, for customer {}",
        account_id, customer_id
    );

    Ok(format!(
        "Trying to create account {}, for customer {}",
        account_id, customer_id,
    ))
}

pub async fn get_accounts(path: web::Path<u32>) -> Result<String> {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id,);

    Ok(format!(
        "Trying to get accounts for customer {}",
        customer_id,
    ))
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
