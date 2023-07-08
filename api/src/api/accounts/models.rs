use serde::Serialize;

#[derive(Serialize)]
pub struct AccountRest {
    pub id: i32,
    pub customer_id: i32,
    pub balance: i32,
}

#[derive(Serialize)]
pub struct AccountsRest {
    pub accounts: Vec<AccountRest>,
}