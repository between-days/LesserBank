// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_status"))]
    pub struct AccountStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_type"))]
    pub struct AccountType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_status"))]
    pub struct TransactionStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
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
        account_name -> Nullable<Varchar>,
        available_balance_cents -> Int8,
        #[max_length = 9]
        account_number -> Varchar,
        bsb -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;
    use super::sql_types::TransactionStatus;

    transactions (id) {
        id -> Int4,
        customer_id -> Int4,
        transaction_type -> TransactionType,
        from_us -> Bool,
        amount_cents -> Int8,
        #[max_length = 9]
        from_number -> Varchar,
        #[max_length = 6]
        from_bsb -> Varchar,
        #[max_length = 40]
        from_name -> Nullable<Varchar>,
        #[max_length = 9]
        to_number -> Varchar,
        #[max_length = 6]
        to_bsb -> Varchar,
        #[max_length = 40]
        to_name -> Nullable<Varchar>,
        available_balance_cents -> Int8,
        date_start -> Timestamptz,
        date_end -> Nullable<Timestamptz>,
        transaction_status -> TransactionStatus,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
