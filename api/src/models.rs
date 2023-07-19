// db models. For now these are also domain models

use crate::schema::accounts;
use diesel::prelude::*;

#[derive(diesel_derive_enum::DbEnum, Copy, Clone, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AccountType"]
pub enum AccountType {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(diesel_derive_enum::DbEnum, Copy, Clone, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AccountStatus"]
pub enum AccountStatus {
    Active,
    Inactive,
}

#[derive(Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountType,
    pub available_balance_cents: i64,
    pub name: Option<String>,
    pub date_opened: chrono::NaiveDateTime,
    pub account_status: AccountStatus,
    pub account_number: String,
    pub bsb: i32,
}

#[derive(Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountType,
    pub name: Option<String>,
    pub available_balance_cents: i64,
    pub account_number: String,
}
