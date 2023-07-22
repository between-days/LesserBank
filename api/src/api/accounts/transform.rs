use crate::models::{Account, AccountStatus, AccountType, NewAccount};

use super::models::{
    AccountRest, AccountStatusRest, AccountTypeRest, AccountsRest, NewAccountRest,
};

impl From<AccountType> for AccountTypeRest {
    fn from(account_type: AccountType) -> Self {
        match account_type {
            AccountType::Savings => AccountTypeRest::Savings,
            AccountType::Transaction => AccountTypeRest::Transaction,
            AccountType::TermDeposit => AccountTypeRest::TermDeposit,
        }
    }
}

impl From<AccountStatus> for AccountStatusRest {
    fn from(account_status: AccountStatus) -> Self {
        match account_status {
            AccountStatus::Active => AccountStatusRest::Active,
            AccountStatus::Inactive => AccountStatusRest::Inactive,
        }
    }
}

impl From<Account> for AccountRest {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            customer_id: account.customer_id,
            balance_cents: account.balance_cents,
            account_type: account.account_type.into(),
            date_opened: account.date_opened.to_string(),
            account_status: account.account_status.into(),
            account_number: account.account_number,
            available_balance_cents: account.available_balance_cents,
            name: account.name,
            bsb: account.bsb,
        }
    }
}

// todo: surely there's a better way to do this
// the reason we don't just pass account like *acc
// is because strings aren't copyable so this whole struct isn't copyable
// so we need to explicitly make a new account with the _explicitly_ copied string
// surely there's a better way to do this, for now it'll do
impl From<Vec<Account>> for AccountsRest {
    fn from(accounts: Vec<Account>) -> Self {
        Self {
            accounts: accounts
                .iter()
                .map(|acc| {
                    AccountRest::from(Account {
                        id: acc.id,
                        customer_id: acc.customer_id,
                        balance_cents: acc.balance_cents,
                        account_type: acc.account_type.into(),
                        date_opened: acc.date_opened,
                        account_status: acc.account_status.into(),
                        account_number: acc.account_number.clone(),
                        available_balance_cents: acc.available_balance_cents,
                        name: acc.name.clone(),
                        bsb: acc.bsb,
                    })
                })
                .collect(),
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
            balance_cents: account.balance_cents,
            account_type: account.account_type.into(),
            name: account.name,
            available_balance_cents: account.available_balance_cents,
            // database will only allow a 9 digit number
            account_number: "".to_string(),
        }
    }
}
