// shared trait definitions

use crate::{
    error::RepoError,
    models::{Account, FindAccountQuery, NewAccount},
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait AccountsRepository: 'static + Sync + Send {
    fn create_account(&self, new_account: NewAccount) -> Result<Account, RepoError>;

    fn get_account(&self, account_id: i32) -> Result<Account, RepoError>;

    fn find_accounts(&self, account_query: FindAccountQuery) -> Result<Vec<Account>, RepoError>;

    fn delete_account(&self, account_id: i32) -> Result<(), RepoError>;
}
