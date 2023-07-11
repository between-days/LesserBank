use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{models::Account, repository::error::RepoError};

pub trait AccountsRepository {
    fn create_account(
        &self,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        customer_id: i32,
    ) -> Account;

    fn get_accounts(
        &self,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        customer_id: i32,
    ) -> Vec<Account>;

    fn get_account(
        &self,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        customer_id: i32,
        account_id: i32,
    ) -> Result<Account, RepoError>;

    fn delete_account(
        &self,
        db_conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        customer_id: i32,
        account_id: i32,
    );
}
