use diesel::{Insertable, Queryable, Selectable};

use crate::schema::transactions;

#[derive(diesel_derive_enum::DbEnum, Copy, Clone, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TransactionStatus"]
pub enum TransactionStatus {
    Pending,
    Success,
    Error,
}

#[derive(diesel_derive_enum::DbEnum, Copy, Clone, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TransactionType"]
pub enum TransactionType {
    Internal,
    External,
}

#[derive(Clone, Queryable, Selectable)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub customer_id: i32,
    pub transaction_type: TransactionType,
    pub from_us: bool,
    pub amount_cents: i64,
    pub from_number: String,
    pub from_bsb: String,
    pub from_name: Option<String>,
    pub to_number: String,
    pub to_bsb: String,
    pub to_name: Option<String>,
    pub available_balance_cents: i64,
    pub date_start: chrono::NaiveDateTime,
    pub date_end: Option<chrono::NaiveDateTime>,
    pub transaction_status: TransactionStatus,
}

#[derive(Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub customer_id: i32,
    pub transaction_type: TransactionType,
    pub from_us: bool,
    pub amount_cents: i64,
    pub from_number: String,
    pub from_bsb: String,
    pub from_name: Option<String>,
    pub to_number: String,
    pub to_bsb: String,
    pub to_name: Option<String>,
    pub available_balance_cents: i64,
    // pub date_start: chrono::NaiveDateTime, // set by db
    pub transaction_status: TransactionStatus,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct FindTransactionQuery {
    pub transaction_id: Option<i32>,
    pub customer_id: i32,
    pub account_number: Option<String>,
    // from number
    // date range?
}
