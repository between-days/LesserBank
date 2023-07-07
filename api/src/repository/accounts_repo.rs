use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use lesser_bank_api::{
    models::{Account, NewAccount},
    schema::{self, accounts},
};

// pub enum RepoError {
//     NotFound,
//     Other,
// }

pub fn create_account(
    db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
) -> Account {
    const ZERO: i32 = 0;

    let new_account = NewAccount {
        customer_id,
        balance: ZERO,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .returning(Account::as_returning())
        .get_result(db_conn)
        .expect("Error saving new account")
}

pub fn get_accounts(
    db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
) -> Vec<Account> {
    accounts::table
        .filter(schema::accounts::customer_id.eq(customer_id))
        .limit(50)
        .select(Account::as_select())
        .load(db_conn)
        .expect("Error loading accounts")
}

pub fn get_account(
    db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
    account_id: i32,
) -> Account {
    //Result<Account, RepoError> {
    accounts::table
        .filter(accounts::customer_id.eq(customer_id))
        .filter(accounts::id.eq(account_id))
        .select(Account::as_select())
        .get_result(db_conn)
        .expect(&format!(
            "Error getting account {} from customer {}",
            account_id, customer_id
        ))

    // match res {
    //     Ok(acc) => Ok(acc),
    //     Err(diesel::result::Error::NotFound) => Err(RepoError::NotFound),
    //     Err(e) => Err(RepoError::Other)
    // }
}

pub fn delete_account(
    db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
    account_id: i32,
) {
    diesel::delete(
        accounts::table
            .filter(accounts::customer_id.eq(customer_id))
            .filter(accounts::id.eq(account_id)),
    )
    .execute(db_conn)
    .expect("Failed to delete account");
}
