// db models. For now these are also domain models

use diesel::prelude::*;

use crate::schema::accounts;

use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Deserialize, Copy, Clone, Debug, PartialEq, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::AccountType"]
pub enum AccountType {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(Copy, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub customer_id: i32,
    pub balance: i32,
    pub account_type: AccountType,
}

#[derive(Insertable, Deserialize, Debug, PartialEq, Copy, Clone)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub customer_id: i32,
    pub balance: i32,
    pub account_type: AccountType,
}
