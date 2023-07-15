// shared trait definitions

use crate::{error::RepoError, models::Account};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait AccountsRepository: 'static + Sync + Send {
    fn create_account(&self, customer_id: i32) -> Result<Account, RepoError>;

    fn get_accounts(&self, customer_id: i32) -> Result<Vec<Account>, RepoError>;

    fn get_account(&self, customer_id: i32, account_id: i32) -> Result<Account, RepoError>;

    fn delete_account(&self, customer_id: i32, account_id: i32) -> Result<(), RepoError>;
}
