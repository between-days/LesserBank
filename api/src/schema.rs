// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_status"))]
    pub struct AccountStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_type"))]
    pub struct AccountType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountType;
    use super::sql_types::AccountStatus;

    accounts (id) {
        id -> Int4,
        customer_id -> Int4,
        account_type -> AccountType,
        balance_cents -> Int8,
        date_opened -> Timestamptz,
        account_status -> AccountStatus,
        #[max_length = 40]
        name -> Nullable<Varchar>,
        available_balance_cents -> Int8,
        #[max_length = 9]
        account_number -> Varchar,
        bsb -> Int4,
    }
}
