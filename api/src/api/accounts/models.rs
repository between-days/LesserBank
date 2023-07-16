use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum AccountTypeRest {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(Serialize)]
pub struct AccountRest {
    pub id: i32,
    pub customer_id: i32,
    pub balance: i32,
    pub account_type: AccountTypeRest,
}

#[derive(Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct NewAccountRest {
    pub customer_id: i32,
    pub balance: i32,
    pub account_type: AccountTypeRest,
}

#[derive(Serialize)]
pub struct AccountsRest {
    pub accounts: Vec<AccountRest>,
}
