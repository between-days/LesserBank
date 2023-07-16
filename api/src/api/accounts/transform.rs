use crate::models::{Account, AccountType, NewAccount};

use super::models::{AccountRest, AccountTypeRest, AccountsRest, NewAccountRest};

impl From<AccountType> for AccountTypeRest {
    fn from(account_type: AccountType) -> Self {
        match account_type {
            AccountType::Savings => AccountTypeRest::Savings,
            AccountType::Transaction => AccountTypeRest::Transaction,
            AccountType::TermDeposit => AccountTypeRest::TermDeposit,
        }
    }
}

impl From<Account> for AccountRest {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            customer_id: account.customer_id,
            balance: account.balance,
            account_type: account.account_type.into(),
        }
    }
}

impl From<Vec<Account>> for AccountsRest {
    fn from(accounts: Vec<Account>) -> Self {
        Self {
            accounts: accounts.iter().map(|acc| AccountRest::from(*acc)).collect(),
        }
    }
}

impl From<AccountTypeRest> for AccountType {
    fn from(account_type: AccountTypeRest) -> Self {
        match account_type {
            AccountTypeRest::Savings => AccountType::Savings,
            AccountTypeRest::Transaction => AccountType::Transaction,
            AccountTypeRest::TermDeposit => AccountType::TermDeposit,
        }
    }
}

impl From<NewAccountRest> for NewAccount {
    fn from(account: NewAccountRest) -> Self {
        NewAccount {
            customer_id: account.customer_id,
            balance: account.balance,
            account_type: account.account_type.into(),
        }
    }
}

impl From<NewAccount> for NewAccountRest {
    fn from(account: NewAccount) -> Self {
        NewAccountRest {
            customer_id: account.customer_id,
            balance: account.balance,
            account_type: account.account_type.into(),
        }
    }
}
