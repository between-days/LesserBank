use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionTypeRest {
    Internal,
    External,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatusRest {
    Pending,
    Success,
    Error,
}

#[cfg_attr(test, derive(Deserialize, PartialEq, Debug))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRest {
    pub id: i32,
    pub customer_id: i32,
    pub transaction_type: TransactionTypeRest,
    pub from_us: bool,
    pub amount_cents: i64,
    pub from_number: String,
    pub from_bsb: String,
    pub from_name: Option<String>,
    pub to_number: String,
    pub to_bsb: String,
    pub to_name: Option<String>,
    pub available_balance_cents: i64,
    pub date_start: String,
    pub date_end: Option<String>,
    pub transaction_status: TransactionStatusRest,
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsRest {
    pub transactions: Vec<TransactionRest>,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewInternalTransactionRest {
    pub customer_id: i32,
    pub amount_cents: i64,
    pub from_number: String,
    pub from_bsb: String,
    pub to_number: String,
    pub to_bsb: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindTransactionQueryRest {
    pub transaction_id: Option<i32>,
    // TODO: consider removing
    // if queries come from other than person owning account this is useful but i doubt we'll use it
    pub customer_id: Option<i32>,
    pub account_number: Option<String>,
}
