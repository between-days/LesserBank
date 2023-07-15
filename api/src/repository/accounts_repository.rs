use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use crate::{
    error::RepoError,
    models::{Account, NewAccount},
    schema::{self, accounts},
    traits::AccountsRepository,
};

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
    fn create_account(&self, customer_id: i32) -> Result<Account, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        let new_account = NewAccount {
            customer_id,
            balance: 0,
        };
        Ok(diesel::insert_into(accounts::table)
            .values(&new_account)
            .returning(Account::as_returning())
            .get_result(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }

    fn get_accounts(&self, customer_id: i32) -> Result<Vec<Account>, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        Ok(accounts::table
            .filter(schema::accounts::customer_id.eq(customer_id))
            .limit(50)
            .select(Account::as_select())
            .load(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }

    fn get_account(&self, customer_id: i32, account_id: i32) -> Result<Account, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        Ok(accounts::table
            .filter(accounts::customer_id.eq(customer_id))
            .filter(accounts::id.eq(account_id))
            .select(Account::as_select())
            .get_result(&mut conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::Other,
            })?)
    }

    fn delete_account(&self, customer_id: i32, account_id: i32) -> Result<(), RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        diesel::delete(
            accounts::table
                .filter(accounts::customer_id.eq(customer_id))
                .filter(accounts::id.eq(account_id)),
        )
        .execute(&mut conn)
        .map_err(|_| RepoError::Other)?;

        Ok(())
    }
}
