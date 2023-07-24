// db models. For now these are also domain models

use crate::traits::{AccountsRepository, TransactionsRepository};

pub mod account;
pub mod transaction;

pub struct AppState<AR: AccountsRepository, TR: TransactionsRepository> {
    pub accounts_repo: AR,
    pub transactions_repo: TR,
}
