impl From<TransactionType> for TransactionTypeRest {
    fn from(transaction_type: TransactionType) -> Self {
        match transaction_type {
            TransactionType::Internal => TransactionTypeRest::Internal,
            TransactionType::External => TransactionTypeRest::External,
        }
    }
}

impl From<TransactionStatus> for TransactionStatusRest {
    fn from(transaction_status: TransactionStatus) -> Self {
        match transaction_status {
            TransactionStatus::Pending => TransactionStatusRest::Pending,
            TransactionStatus::Success => TransactionStatusRest::Success,
            TransactionStatus::Error => TransactionStatusRest::Error,
        }
    }
}

fn string_opt_from_naive_dt_opt(dt: Option<NaiveDateTime>) -> Option<String> {
    match dt {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

impl From<Transaction> for TransactionRest {
    fn from(tr: Transaction) -> Self {
        Self {
            id: tr.id,
            customer_id: tr.customer_id,
            transaction_type: tr.transaction_type.into(),
            from_us: tr.from_us,
            amount_cents: tr.amount_cents,
            from_number: tr.from_number,
            from_bsb: tr.from_bsb,
            from_name: tr.from_name,
            to_number: tr.to_number,
            to_bsb: tr.to_bsb,
            to_name: tr.to_name,
            available_balance_cents: tr.available_balance_cents,
            date_start: tr.date_start.to_string(),
            date_end: string_opt_from_naive_dt_opt(tr.date_end),
            transaction_status: tr.transaction_status.into(),
        }
    }
}

use chrono::NaiveDateTime;

use crate::models::transaction::{NewTransaction, Transaction, TransactionStatus, TransactionType};

use super::models::{
    NewInternalTransactionRest, TransactionRest, TransactionStatusRest, TransactionTypeRest,
    TransactionsRest,
};

// // todo: surely there's a better way to do this
// // the reason we don't just pass account like *acc
// // is because strings aren't copyable so this whole struct isn't copyable
// // so we need to explicitly make a new account with the _explicitly_ copied string
// // surely there's a better way to do this, for now it'll do
impl From<Vec<Transaction>> for TransactionsRest {
    fn from(transactions: Vec<Transaction>) -> Self {
        Self {
            transactions: transactions
                .iter()
                .map(|tr| {
                    TransactionRest::from(Transaction {
                        id: tr.id,
                        customer_id: tr.customer_id,
                        transaction_type: tr.transaction_type,
                        from_us: tr.from_us,
                        amount_cents: tr.amount_cents,
                        from_number: tr.from_number.clone(),
                        from_bsb: tr.from_bsb.clone(),
                        from_name: tr.from_name.clone(),
                        to_number: tr.to_number.clone(),
                        to_bsb: tr.to_bsb.clone(),
                        to_name: tr.to_name.clone(),
                        available_balance_cents: tr.available_balance_cents,
                        date_start: tr.date_start,
                        date_end: tr.date_end,
                        transaction_status: tr.transaction_status,
                    })
                })
                .collect(),
        }
    }
}

impl From<TransactionTypeRest> for TransactionType {
    fn from(tr_type: TransactionTypeRest) -> Self {
        match tr_type {
            TransactionTypeRest::Internal => TransactionType::Internal,
            TransactionTypeRest::External => TransactionType::External,
        }
    }
}

impl From<TransactionStatusRest> for TransactionStatus {
    fn from(tr_status: TransactionStatusRest) -> Self {
        match tr_status {
            TransactionStatusRest::Pending => TransactionStatus::Pending,
            TransactionStatusRest::Success => TransactionStatus::Success,
            TransactionStatusRest::Error => TransactionStatus::Error,
        }
    }
}

impl From<NewInternalTransactionRest> for NewTransaction {
    fn from(tr: NewInternalTransactionRest) -> Self {
        NewTransaction {
            customer_id: tr.customer_id,
            transaction_type: TransactionType::Internal,
            from_us: true,
            amount_cents: tr.amount_cents,
            from_number: tr.from_number,
            from_bsb: tr.from_bsb,
            from_name: Some("".to_string()),
            to_number: tr.to_number,
            to_bsb: tr.to_bsb,
            to_name: Some("".to_string()),
            available_balance_cents: 0,
            date_start: chrono::NaiveDateTime::from_timestamp_millis(0).unwrap(),
            transaction_status: TransactionStatus::Pending,
        }
    }
}
