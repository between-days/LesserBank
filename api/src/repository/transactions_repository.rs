use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use crate::{
    error::RepoError,
    models::transaction::{FindTransactionQuery, NewTransaction, Transaction},
    schema::{self, transactions},
    traits::{RepoCreate, RepoFind},
};

#[derive(Clone)]
pub struct TransactionsRepoImpl {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl TransactionsRepoImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> TransactionsRepoImpl {
        TransactionsRepoImpl { pool }
    }
}

impl RepoCreate<Transaction, NewTransaction> for TransactionsRepoImpl {
    fn create(&self, new_transaction: NewTransaction) -> Result<Transaction, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        Ok(diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .returning(Transaction::as_returning())
            .get_result(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }
}

impl RepoFind<Transaction, FindTransactionQuery> for TransactionsRepoImpl {
    fn find(&self, transaction_query: FindTransactionQuery) -> Result<Vec<Transaction>, RepoError> {
        let mut conn = self.pool.get().map_err(|_| {
            println!("couldn't get db connection from pool");
            RepoError::ConnectionError
        })?;

        let mut query = schema::transactions::table.into_boxed();
        if let Some(id) = transaction_query.transaction_id {
            query = query.filter(transactions::id.eq(id));
        }

        query = query.filter(transactions::customer_id.eq(transaction_query.customer_id));

        if let Some(account_number) = transaction_query.account_number {
            query = query.filter(
                transactions::from_number
                    .eq(account_number.clone())
                    .or(transactions::to_number.eq(account_number)),
            );
        }

        Ok(query
            .limit(50)
            .select(Transaction::as_select())
            .load(&mut conn)
            .map_err(|_| RepoError::Other)?)
    }
}
