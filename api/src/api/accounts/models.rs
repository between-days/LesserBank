use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum AccountTypeRest {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum AccountStatusRest {
    Active,
    Inactive,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize)]
pub struct AccountRest {
    pub id: i32,
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountTypeRest,
    pub date_opened: String,
    pub account_status: AccountStatusRest,
    pub account_number: String,
    pub available_balance_cents: i64,
    pub name: Option<String>,
    pub bsb: i32,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NewAccountRest {
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountTypeRest,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct AccountsRest {
    pub accounts: Vec<AccountRest>,
}
