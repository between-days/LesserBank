use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path};
use actix_web::{web, HttpResponse};
use rand::Rng;

use super::error::AccountsApiError;
use super::models::{AccountRest, AccountsRest};
use crate::error::RepoError;
use crate::traits::AccountsRepository;

pub async fn create_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
) -> Result<HttpResponse, AccountsApiError> {
    let customer_id = path.into_inner();
    let account_id = rand::thread_rng().gen_range(0..100);

    println!(
        "Trying to create account {} for customer {}",
        account_id, customer_id
    );

    let account = web::block(move || accounts_repo.create_account(customer_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    Ok(HttpResponse::Created()
        .insert_header(ContentType::json())
        .json(web::Json(AccountRest {
            id: account.id,
            customer_id: account.customer_id,
            balance: account.balance,
        })))
}

pub async fn get_accounts<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
) -> Result<HttpResponse, AccountsApiError> {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let accounts = web::block(move || accounts_repo.get_accounts(customer_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(web::Json(AccountsRest {
            accounts: accounts
                .iter()
                .map(|acc| AccountRest {
                    id: acc.id,
                    customer_id: acc.customer_id,
                    balance: acc.balance,
                })
                .collect(),
        })))
}

pub async fn get_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, AccountsApiError> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    );

    let account = web::block(move || accounts_repo.get_account(customer_id, account_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|err| match err {
            RepoError::NotFound => AccountsApiError::NotFound,
            _ => AccountsApiError::InternalError,
        })?;

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(web::Json(AccountRest {
            id: account.id,
            customer_id: account.customer_id,
            balance: account.balance,
        })))
}

pub async fn delete_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, AccountsApiError> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    );

    web::block(move || accounts_repo.delete_account(customer_id, account_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    Ok(HttpResponse::NoContent().body(""))
}

#[cfg(test)]
mod tests {
    use crate::{
        api::accounts::{
            handlers::{create_account, delete_account, get_account, get_accounts},
            models::{AccountRest, AccountsRest},
        },
        models::Account,
        traits::MockAccountsRepository,
    };

    use actix_web::{body::to_bytes, http::StatusCode, web::Data};
    use mockall::predicate::eq;

    #[actix_web::test]
    async fn test_create_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_create_account()
            .with(eq(customer_id))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance: 3,
                })
            });

        let res = create_account(Data::new(mock_repo), customer_id.into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::CREATED;
        let expected_body = serde_json::to_string(&AccountRest {
            id: account_id,
            customer_id,
            balance: 3,
        })
        .unwrap();

        assert_eq!(expected_status, actual_status);
        assert_eq!(expected_body, actual_body)
    }

    #[actix_web::test]
    async fn test_get_accounts_success() {
        let customer_id = 1;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_accounts()
            .with(eq(customer_id))
            .times(1)
            .returning(move |_| {
                Ok(vec![
                    Account {
                        id: 1,
                        customer_id: 1,
                        balance: 4,
                    },
                    Account {
                        id: 2,
                        customer_id: 1,
                        balance: 5,
                    },
                ])
            });

        let res = get_accounts(Data::new(mock_repo), customer_id.into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::OK;
        let expected_body = serde_json::to_string(&AccountsRest {
            accounts: vec![
                AccountRest {
                    id: 1,
                    customer_id: 1,
                    balance: 4,
                },
                AccountRest {
                    id: 2,
                    customer_id: 1,
                    balance: 5,
                },
            ],
        })
        .unwrap();

        assert_eq!(expected_status, actual_status);
        assert_eq!(expected_body, actual_body)
    }

    #[actix_web::test]
    async fn test_get_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(customer_id), eq(account_id))
            .times(1)
            .returning(move |customer_id, id| {
                Ok(Account {
                    id,
                    customer_id,
                    balance: 3,
                })
            });

        let res = get_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::OK;
        let expected_body = serde_json::to_string(&AccountRest {
            id: account_id,
            customer_id,
            balance: 3,
        })
        .unwrap();

        assert_eq!(expected_status, actual_status);
        assert_eq!(expected_body, actual_body)
    }

    #[actix_web::test]
    async fn test_delete_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_delete_account()
            .with(eq(customer_id), eq(account_id))
            .times(1)
            .returning(|_, _| Ok(()));

        let res = delete_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::NO_CONTENT;

        assert_eq!(expected_status, actual_status);
        assert_eq!("", actual_body)
    }
}
