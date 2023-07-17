// db models. For now these are also domain models

use diesel::prelude::*;

use crate::schema::accounts;

#[derive(diesel_derive_enum::DbEnum, Copy, Clone, Debug, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AccountType"]
pub enum AccountType {
    Savings,
    Transaction,
    TermDeposit,
}

#[derive(Copy, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountType,
}

#[derive(Insertable, Debug, PartialEq, Copy, Clone)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub customer_id: i32,
    pub balance_cents: i64,
    pub account_type: AccountType,
}
