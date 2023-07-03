use diesel::prelude::*;
use lesser_bank_api::{
    establish_connection,
    models::{Account, NewAccount},
    schema::accounts,
};

pub fn _create_account(customer_id: i32) -> Account {
    let connection = &mut establish_connection();

    let zero = 0;
    let new_account = NewAccount {
        customer_id,
        balance: zero,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .returning(Account::as_returning())
        .get_result(connection)
        .expect("Error saving new account")
}

pub fn _get_accounts() -> Vec<Account> {
    let connection = &mut establish_connection();

    let results: Vec<Account> = accounts::table
        .limit(50)
        .select(Account::as_select())
        .load(connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());

    let mut accs: Vec<Account> = Vec::new();

    for account in results {
        println!("id {}", account.id.to_string());
        println!("-----------\n");
        println!("customer_id {}", account.customer_id.to_string());
        accs.push(account);
    }

    accs
}
