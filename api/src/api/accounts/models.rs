use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AccountTypeRest {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AccountStatusRest {
    Active,
    Inactive,
}

#[cfg_attr(test, derive(Deserialize, PartialEq, Debug))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct NewAccountRest {
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountTypeRest,
    pub name: Option<String>,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsRest {
    pub accounts: Vec<AccountRest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindAccountQueryRest {
    pub account_id: Option<i32>,
    pub customer_id: Option<i32>,
    pub account_number: Option<String>,
}
