use crate::{models::Account, repository::error::RepoError};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait AccountsRepository: 'static + Sync + Send {
    fn create_account(&self, customer_id: i32) -> Account;

    fn get_accounts(&self, customer_id: i32) -> Vec<Account>;

    fn get_account(&self, customer_id: i32, account_id: i32) -> Result<Account, RepoError>;

    fn delete_account(&self, customer_id: i32, account_id: i32);
}
