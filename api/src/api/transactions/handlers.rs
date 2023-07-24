use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path, Query};
use actix_web::{web, HttpResponse};

use super::models::{FindTransactionQueryRest, NewInternalTransactionRest};
use crate::api::error::ApiError;
use crate::api::transactions::models::{TransactionRest, TransactionsRest};
use crate::models::account::FindAccountQuery;
use crate::models::transaction::{FindTransactionQuery, NewTransaction};
use crate::models::AppState;
use crate::traits::{AccountsRepository, TransactionsRepository};

pub async fn new_internal_transaction<AR: AccountsRepository, TR: TransactionsRepository>(
    app_state: Data<AppState<AR, TR>>,
    path: Path<i32>,
    payload: web::Json<NewInternalTransactionRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    let mut new_transaction: NewTransaction = payload.into_inner().into();

    if new_transaction.customer_id != customer_id {
        return Err(ApiError::Unauthorized.into());
    }

    let account_query = FindAccountQuery {
        customer_id,
        account_id: None,
        account_number: Some(new_transaction.from_number.clone()),
    };

    println!(
        "Trying to find related account {:?} for customer {}",
        account_query.account_number, customer_id
    );

    // TODO: !! BLOCKING !!
    let accounts = app_state
        .accounts_repo
        .find_accounts(account_query.clone())
        .map_err(|_| ApiError::BadRequest)?;

    if accounts.len() > 1 {
        return Err(ApiError::InternalError.into());
    } else if accounts.len() != 1 {
        println!(
            "Couldn't find related account {:?} for customer {}",
            account_query.account_number, customer_id
        );
        return Err(ApiError::BadRequest.into());
    }

    let account_from = &accounts[0];

    if account_from.available_balance_cents < new_transaction.amount_cents {
        println!("available balance would be negative");
        return Err(ApiError::BadRequest.into());
    }
    new_transaction.available_balance_cents =
        account_from.available_balance_cents - new_transaction.amount_cents;

    println!(
        "Trying to create {:?} transaction for customer {}",
        new_transaction.transaction_type, customer_id
    );

    // TODO: !! BLOCKING !!
    let transaction = app_state
        .transactions_repo
        .create_transaction(new_transaction)
        .map_err(|_| ApiError::InternalError)?;

    // TODO: put job in queue to call handler /webhook/transaction/execute-transaction
    // that will get second account and move the money around

    Ok(HttpResponse::Created()
        .insert_header(ContentType::json())
        .json(web::Json::<TransactionRest>(transaction.into())))
}

pub async fn find_transactions<AR: AccountsRepository, TR: TransactionsRepository>(
    app_state: Data<AppState<AR, TR>>,
    path: Path<i32>,
    query: Query<FindTransactionQueryRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    let query = FindTransactionQuery {
        transaction_id: query.transaction_id,
        customer_id,
        account_number: query.account_number.clone(),
    };

    if query.customer_id != customer_id {
        return Err(ApiError::Unauthorized.into());
    }

    println!("Trying to get transactions for customer {}", customer_id);

    let transactions = web::block(move || app_state.transactions_repo.find_transactions(query))
        .await
        .map_err(|_| ApiError::InternalError)?
        .map_err(|_| ApiError::InternalError)?;

    // enforced query on customer_id so should always be fine, but safe > sorry
    for acc in transactions.iter() {
        if acc.customer_id != customer_id {
            return Err(ApiError::BadRequest.into());
        }
    }

    println!("Got transactions for customer {}", customer_id);

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(web::Json::<TransactionsRest>(transactions.into())))
}
