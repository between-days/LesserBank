use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path};
use actix_web::{web, HttpResponse};

use super::error::AccountsApiError;
use super::models::{AccountRest, AccountsRest, NewAccountRest};

use crate::error::RepoError;
use crate::models::NewAccount;
use crate::traits::AccountsRepository;

pub async fn create_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
    payload: web::Json<NewAccountRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    let new_account: NewAccount = payload.into_inner().into();

    if new_account.customer_id != customer_id {
        return Err(AccountsApiError::Unauthorized.into());
    }

    println!(
        "Trying to create {:?} account for customer {}",
        new_account.account_type, customer_id
    );

    let account = web::block(move || accounts_repo.create_account(new_account))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    Ok(HttpResponse::Created()
        .insert_header(ContentType::json())
        .json(web::Json::<AccountRest>(account.into())))
}

pub async fn get_accounts_for_customer<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    println!("Trying to get accounts for customer {}", customer_id);

    let accounts = web::block(move || accounts_repo.get_accounts_by_customer(customer_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(web::Json::<AccountsRest>(accounts.into())))
}

pub async fn get_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::Error> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to get account {}, for customer {}",
        account_id, customer_id
    );

    let account = web::block(move || accounts_repo.get_account(account_id))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|err| match err {
            RepoError::NotFound => AccountsApiError::NotFound,
            _ => AccountsApiError::InternalError,
        })?;

    if account.customer_id != customer_id {
        return Err(AccountsApiError::Unauthorized.into());
    }

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(web::Json::<AccountRest>(account.into())))
}

pub async fn delete_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<(i32, i32)>,
) -> Result<HttpResponse, actix_web::Error> {
    let (customer_id, account_id) = path.into_inner();

    println!(
        "Trying to delete account {}, for customer {}",
        account_id, customer_id
    );

    web::block(move || {
        let get_acc_res = accounts_repo.get_account(account_id);

        let check_account_error: Option<AccountsApiError> = match get_acc_res {
            Ok(acc) => {
                if acc.customer_id != customer_id {
                    Some(AccountsApiError::Unauthorized)
                } else {
                    None
                }
            }
            Err(RepoError::NotFound) => None,
            _ => Some(AccountsApiError::InternalError),
        };

        if check_account_error.is_some() {
            let cc = check_account_error.unwrap();
            return Err(cc);
        }

        accounts_repo
            .delete_account(account_id)
            .map_err(|_| AccountsApiError::InternalError)
    })
    .await
    .map_err(|_| AccountsApiError::InternalError)?
    .map_err(|err| err)?;

    Ok(HttpResponse::NoContent().body(""))
}

#[cfg(test)]
mod tests {
    use crate::{
        api::accounts::{
            error::AccountsApiError,
            handlers::{create_account, delete_account, get_account, get_accounts_for_customer},
            models::{AccountRest, AccountTypeRest, AccountsRest, NewAccountRest},
        },
        error::RepoError,
        models::{Account, AccountType, NewAccount},
        traits::MockAccountsRepository,
    };

    use actix_web::{
        body::to_bytes,
        http::StatusCode,
        web::{Data, Json},
    };
    use mockall::{predicate::eq, Sequence};

    #[actix_web::test]
    async fn test_create_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let new_account_rest = NewAccountRest {
            customer_id,
            balance: 1,
            account_type: AccountTypeRest::Savings,
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_create_account()
            .with(eq::<NewAccount>(new_account_rest.into()))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance: 3,
                    account_type: AccountType::Savings,
                })
            });

        let res = create_account(
            Data::new(mock_repo),
            customer_id.into(),
            Json(new_account_rest),
        )
        .await
        .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::CREATED;
        let expected_body = serde_json::to_string(&AccountRest {
            id: account_id,
            customer_id,
            balance: 3,
            account_type: AccountTypeRest::Savings,
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
            .expect_get_accounts_by_customer()
            .with(eq(customer_id))
            .times(1)
            .returning(move |_| {
                Ok(vec![
                    Account {
                        id: 1,
                        customer_id: 1,
                        balance: 4,
                        account_type: AccountType::Savings,
                    },
                    Account {
                        id: 2,
                        customer_id: 1,
                        balance: 5,
                        account_type: AccountType::Savings,
                    },
                ])
            });

        let res = get_accounts_for_customer(Data::new(mock_repo), customer_id.into())
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
                    account_type: AccountTypeRest::Savings,
                },
                AccountRest {
                    id: 2,
                    customer_id: 1,
                    balance: 5,
                    account_type: AccountTypeRest::Savings,
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

        let account = Account {
            id: account_id,
            customer_id,
            balance: 3,
            account_type: AccountType::Savings,
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Ok(account));

        let res = get_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::OK;
        let expected_body = serde_json::to_string::<AccountRest>(&account.into()).unwrap();

        assert_eq!(expected_status, actual_status);
        assert_eq!(expected_body, actual_body)
    }

    #[actix_web::test]
    async fn test_delete_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let account = Account {
            id: account_id,
            customer_id,
            balance: 3,
            account_type: AccountType::Savings,
        };

        let mut seq = Sequence::new();

        let mut mock_repo = MockAccountsRepository::new();

        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Ok(account))
            .in_sequence(&mut seq);

        mock_repo
            .expect_delete_account()
            .with(eq(account_id))
            .times(1)
            .returning(|_| Ok(()))
            .in_sequence(&mut seq);

        let res = delete_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::NO_CONTENT;

        assert_eq!(expected_status, actual_status);
        assert_eq!("", actual_body)
    }

    #[actix_web::test]
    async fn test_delete_account_not_found_success() {
        let customer_id = 5;
        let account_id = 2;

        let mut seq = Sequence::new();

        let mut mock_repo = MockAccountsRepository::new();

        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Err(crate::error::RepoError::NotFound))
            .in_sequence(&mut seq);

        mock_repo
            .expect_delete_account()
            .with(eq(account_id))
            .times(1)
            .returning(|_| Ok(()))
            .in_sequence(&mut seq);

        let res = delete_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::NO_CONTENT;

        assert_eq!(expected_status, actual_status);
        assert_eq!("", actual_body)
    }

    #[actix_web::test]
    async fn test_delete_account_unauthorized_error() {
        let customer_id = 5;
        let account_id = 2;
        let wrong_customer_id = 2;

        let account = Account {
            id: account_id,
            customer_id,
            balance: 3,
            account_type: AccountType::Savings,
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Ok(account));

        let res =
            delete_account(Data::new(mock_repo), (wrong_customer_id, account_id).into()).await;

        assert!(res.is_err_and(|e| { e.to_string() == AccountsApiError::Unauthorized.to_string() }));
    }
    #[actix_web::test]
    async fn test_delete_account_internal_error() {
        let customer_id = 5;
        let account_id = 2;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Err(RepoError::Other));

        let res = delete_account(Data::new(mock_repo), (customer_id, account_id).into()).await;

        assert!(
            res.is_err_and(|e| { e.to_string() == AccountsApiError::InternalError.to_string() })
        );
    }
}
