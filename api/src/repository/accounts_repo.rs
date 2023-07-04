use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use lesser_bank_api::{
    models::{Account, NewAccount},
    schema::{self, accounts},
};

pub fn create_account(
    db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
) -> Account {
    let zero = 0;
    let new_account = NewAccount {
        customer_id,
        balance: zero,
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
