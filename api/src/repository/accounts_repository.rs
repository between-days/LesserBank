use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use crate::{
    error::RepoError,
    models::account::{Account, FindAccountQuery, NewAccount},
    schema::{self, accounts},
    traits::{RepoCreate, RepoDeleteById, RepoFind, RepoGetById},
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

impl RepoCreate<Account, NewAccount> for AccountsRepoImpl {
    fn create(&self, new_account: NewAccount) -> Result<Account, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        Ok(diesel::insert_into(accounts::table)
            .values(&new_account)
            .returning(Account::as_returning())
            .get_result(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }
}

impl RepoFind<Account, FindAccountQuery> for AccountsRepoImpl {
    fn find(&self, account_query: FindAccountQuery) -> Result<Vec<Account>, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        let mut query = schema::accounts::table.into_boxed();
        if let Some(id) = account_query.account_id {
            query = query.filter(accounts::id.eq(id));
        }

        query = query.filter(accounts::customer_id.eq(account_query.customer_id));

        if let Some(account_number) = account_query.account_number {
            query = query.filter(accounts::account_number.eq(account_number))
        }

        Ok(query
            .limit(50)
            .select(Account::as_select())
            .load(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }
}

impl RepoGetById<Account> for AccountsRepoImpl {
    fn get_by_id(&self, account_id: i32) -> Result<Account, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        Ok(accounts::table
            .filter(accounts::id.eq(account_id))
            .select(Account::as_select())
            .get_result(&mut conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => RepoError::NotFound,
                _ => RepoError::Other,
            })?)
    }
}

impl RepoDeleteById<Account> for AccountsRepoImpl {
    fn delete_by_id(&self, account_id: i32) -> Result<(), RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        diesel::delete(accounts::table.filter(accounts::id.eq(account_id)))
            .execute(&mut conn)
            .map_err(|_| RepoError::Other)?;

        Ok(())
    }
}
