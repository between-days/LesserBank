use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use crate::{
    models::{Account, NewAccount},
    schema::{self, accounts},
    traits::AccountsRepository,
};

use super::super::error::RepoError;

#[derive(Clone)]
pub struct AccountsRepoImpl {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl AccountsRepoImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> AccountsRepoImpl {
        AccountsRepoImpl { pool }
    }
}

impl AccountsRepository for AccountsRepoImpl {
    fn create_account(&self, customer_id: i32) -> Account {
        let new_account = NewAccount {
            customer_id,
            balance: 0,
        };

        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        diesel::insert_into(accounts::table)
            .values(&new_account)
            .returning(Account::as_returning())
            .get_result(&mut conn)
            .expect("Error saving new account")
    }

    fn get_accounts(&self, customer_id: i32) -> Vec<Account> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        accounts::table
            .filter(schema::accounts::customer_id.eq(customer_id))
            .limit(50)
            .select(Account::as_select())
            .load(&mut conn)
            .expect("Error loading accounts")
    }

    fn get_account(&self, customer_id: i32, account_id: i32) -> Result<Account, RepoError> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let res = accounts::table
            .filter(accounts::customer_id.eq(customer_id))
            .filter(accounts::id.eq(account_id))
            .select(Account::as_select())
            .get_result(&mut conn);
        match res {
            Ok(account) => Ok(account),
            Err(diesel::result::Error::NotFound) => Err(RepoError::NotFound),
            Err(_) => Err(RepoError::Other),
        }
    }

    fn delete_account(&self, customer_id: i32, account_id: i32) {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        diesel::delete(
            accounts::table
                .filter(accounts::customer_id.eq(customer_id))
                .filter(accounts::id.eq(account_id)),
        )
        .execute(&mut conn)
        .expect("Failed to delete account");
    }
}
