// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        customer_id -> Int4,
        balance -> Int4,
    }
}
