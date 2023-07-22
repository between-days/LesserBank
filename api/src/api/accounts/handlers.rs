use actix_web::http::header::ContentType;
use actix_web::web::{Data, Path, Query};
use actix_web::{web, HttpResponse};

use super::error::AccountsApiError;
use super::models::{AccountRest, AccountsRest, FindAccountQueryRest, NewAccountRest};

use crate::api::accounts::util::get_random_account_number;
use crate::error::RepoError;
use crate::models::{FindAccountQuery, NewAccount};
use crate::traits::AccountsRepository;

pub async fn create_account<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
    payload: web::Json<NewAccountRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    let mut new_account: NewAccount = payload.into_inner().into();

    if new_account.customer_id != customer_id {
        return Err(AccountsApiError::Unauthorized.into());
    }

    // completely random for now
    let account_number = get_random_account_number();
    new_account.account_number = account_number;

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

pub async fn find_accounts<T: AccountsRepository>(
    accounts_repo: Data<T>,
    path: Path<i32>,
    query: Query<FindAccountQueryRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let customer_id = path.into_inner();

    let query = FindAccountQuery {
        account_id: query.account_id,
        customer_id,
        account_number: query.account_number.clone(),
    };

    if query.customer_id != customer_id {
        return Err(AccountsApiError::Unauthorized.into());
    }

    println!("Trying to get accounts for customer {}", customer_id);

    let accounts = web::block(move || accounts_repo.find_accounts(query))
        .await
        .map_err(|_| AccountsApiError::InternalError)?
        .map_err(|_| AccountsApiError::InternalError)?;

    // enforced query on customer_id so should always be fine, but safe > sorry
    for acc in accounts.iter() {
        if acc.customer_id != customer_id {
            return Err(AccountsApiError::BadRequest.into());
        }
    }

    println!("Got accounts for customer {}", customer_id);

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
    use std::vec;

    use crate::{
        api::accounts::{
            error::AccountsApiError,
            handlers::{create_account, delete_account, find_accounts, get_account},
            models::{
                AccountRest, AccountStatusRest, AccountTypeRest, AccountsRest, NewAccountRest,
            },
        },
        error::RepoError,
        models::{Account, AccountStatus, AccountType, FindAccountQuery, NewAccount},
        traits::MockAccountsRepository,
    };

    use actix_web::{
        body::to_bytes,
        http::StatusCode,
        test,
        web::{self, Data, Json},
        App,
    };
    use chrono::{NaiveDate, NaiveDateTime};
    use mockall::{predicate::eq, Sequence};

    #[actix_web::test]
    async fn test_create_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap();

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_create_account()
            .withf(move |acc| {
                acc.customer_id == customer_id
                    && acc.balance_cents == 34343
                    && acc.account_type == AccountType::Savings
                    && acc.name == Some("abc".to_string())
                    && acc.available_balance_cents == 3444
            })
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance_cents: 13424234234,
                    account_type: AccountType::Savings,
                    date_opened: dt,
                    account_status: AccountStatus::Active,
                    name: Some("abc".to_string()),
                    account_number: "012345678".to_string(),
                    available_balance_cents: 3444,
                    bsb: 123456,
                })
            });

        let mut app = test::init_service(
            App::new().app_data(Data::new(mock_repo)).service(
                web::resource("/customers/{customer_id}/accounts")
                    .route(web::post().to(create_account::<MockAccountsRepository>)),
            ),
        )
        .await;

        let testpayload = NewAccountRest {
            customer_id,
            balance_cents: 34343,
            available_balance_cents: 3444,
            account_type: AccountTypeRest::Savings,
            name: Some("abc".to_string()),
        };

        let resp = test::TestRequest::post()
            .uri("/customers/1/accounts")
            .append_header(("Content-Type", "application/json"))
            .set_payload(serde_json::to_string(&testpayload).unwrap())
            .send_request(&mut app)
            .await;

        let actual_status = resp.status();
        assert_eq!(StatusCode::CREATED, actual_status);

        let expected_account = AccountRest {
            id: account_id,
            customer_id,
            balance_cents: 13424234234,
            account_type: AccountTypeRest::Savings,
            date_opened: dt.to_string(),
            account_status: AccountStatusRest::Active,
            name: Some("abc".to_string()),
            account_number: "012345678".to_string(),
            available_balance_cents: 3444,
            bsb: 123456,
        };

        let actual_account: AccountRest = test::read_body_json(resp).await;
        assert_eq!(expected_account.id, actual_account.id);
        assert_eq!(expected_account.customer_id, actual_account.customer_id);
        assert_eq!(expected_account.balance_cents, actual_account.balance_cents);
        assert_eq!(expected_account.account_type, actual_account.account_type);
        assert_eq!(
            expected_account.date_opened.to_string(),
            actual_account.date_opened
        );
        assert_eq!(
            expected_account.account_status,
            actual_account.account_status
        );
        assert_eq!(expected_account.name, actual_account.name);
        assert_eq!(
            expected_account.available_balance_cents,
            actual_account.available_balance_cents
        );
    }

    #[actix_web::test]
    async fn test_create_account_internal_error() {
        let customer_id = 5;

        let new_account = NewAccount {
            customer_id,
            balance_cents: 13424234234,
            account_type: AccountType::Savings,
            name: Some("abc".to_string()),
            available_balance_cents: 34343,
            account_number: "".to_string(),
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_create_account()
            .with(eq(new_account))
            .times(1)
            .returning(move |_| Err(RepoError::Other));

        let res = create_account(
            Data::new(mock_repo),
            customer_id.into(),
            Json(NewAccountRest {
                customer_id,
                balance_cents: 13424234234,
                available_balance_cents: 3444,
                account_type: AccountTypeRest::Savings,
                name: Some("abc".to_string()),
            }),
        )
        .await;

        assert!(
            res.is_err_and(|e| { e.to_string() == AccountsApiError::InternalError.to_string() })
        );
    }

    #[actix_web::test]
    async fn test_find_accounts_by_customer_id_success() {
        let customer_id = 1;

        let expected_query = FindAccountQuery {
            account_id: None,
            customer_id,
            account_number: None,
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_find_accounts()
            .with(eq(expected_query))
            .times(1)
            .returning(move |_| {
                Ok(vec![
                    Account {
                        id: 1,
                        customer_id: 1,
                        balance_cents: 13424234234,
                        account_type: AccountType::Savings,
                        date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                            .unwrap()
                            .and_hms_opt(9, 10, 11)
                            .unwrap(),
                        account_status: AccountStatus::Active,
                        name: Some("abc".to_string()),
                        available_balance_cents: 34343,
                        account_number: "012345678".to_string(),
                        bsb: 123456,
                    },
                    Account {
                        id: 2,
                        customer_id: 1,
                        balance_cents: 13424234234,
                        account_type: AccountType::Savings,
                        date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                            .unwrap()
                            .and_hms_opt(9, 10, 11)
                            .unwrap(),
                        account_status: AccountStatus::Active,
                        available_balance_cents: 34343,
                        name: Some("abc".to_string()),
                        account_number: "012345678".to_string(),
                        bsb: 123456,
                    },
                ])
            });

        let mut app = test::init_service(
            App::new().app_data(Data::new(mock_repo)).service(
                web::resource("/customers/{customer_id}/accounts")
                    .route(web::get().to(find_accounts::<MockAccountsRepository>)),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri(format!("/customers/{0}/accounts?customerId={0}", customer_id).as_str())
            .append_header(("Content-Type", "application/json"))
            .send_request(&mut app)
            .await;

        let actual_status = resp.status();
        assert_eq!(StatusCode::OK, actual_status);

        let expected_res = AccountsRest {
            accounts: vec![
                AccountRest {
                    id: 1,
                    customer_id: 1,
                    balance_cents: 13424234234,
                    account_type: AccountTypeRest::Savings,
                    date_opened: "2016-07-08 09:10:11".to_string(),
                    account_status: AccountStatusRest::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                },
                AccountRest {
                    id: 2,
                    customer_id: 1,
                    balance_cents: 13424234234,
                    account_type: AccountTypeRest::Savings,
                    date_opened: "2016-07-08 09:10:11".to_string(),
                    account_status: AccountStatusRest::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                },
            ],
        };

        let actual_res: AccountsRest = test::read_body_json(resp).await;
        assert_eq!(expected_res.accounts[0], actual_res.accounts[0]);
        assert_eq!(expected_res.accounts[1], actual_res.accounts[1]);
    }

    #[actix_web::test]
    async fn test_find_accounts_internal_error() {
        let customer_id = 2;

        let expected_query = FindAccountQuery {
            account_id: None,
            customer_id,
            account_number: None,
        };

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_find_accounts()
            .with(eq(expected_query))
            .times(1)
            .returning(move |_| Err(RepoError::Other));

        let mut app = test::init_service(
            App::new().app_data(Data::new(mock_repo)).service(
                web::resource("/customers/{customer_id}/accounts")
                    .route(web::get().to(find_accounts::<MockAccountsRepository>)),
            ),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri(format!("/customers/{0}/accounts?customerId={0}", customer_id).as_str())
            .append_header(("Content-Type", "application/json"))
            .send_request(&mut app)
            .await;

        let actual_status = resp.status();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, actual_status);
    }

    #[actix_web::test]
    async fn test_get_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance_cents: 13424234234,
                    account_type: AccountType::Savings,
                    date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap(),
                    account_status: AccountStatus::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                })
            });

        let res = get_account(Data::new(mock_repo), (customer_id, account_id).into())
            .await
            .unwrap();

        let actual_status = res.status();
        let actual_body = to_bytes(res.into_body()).await.unwrap();

        let expected_status = StatusCode::OK;
        let expected_body = serde_json::to_string::<AccountRest>(&AccountRest {
            id: account_id,
            customer_id,
            balance_cents: 13424234234,
            account_type: AccountTypeRest::Savings,
            date_opened: "2016-07-08 09:10:11".to_string(),
            account_status: AccountStatusRest::Active,
            name: Some("abc".to_string()),
            available_balance_cents: 34343,
            account_number: "012345678".to_string(),
            bsb: 123456,
        })
        .unwrap();

        assert_eq!(expected_status, actual_status);
        assert_eq!(expected_body, actual_body)
    }

    #[actix_web::test]
    async fn test_get_account_internal_error() {
        let customer_id = 1;
        let account_id = 4;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Err(RepoError::Other));

        let res = get_account(Data::new(mock_repo), (customer_id, account_id).into()).await;

        assert!(
            res.is_err_and(|e| { e.to_string() == AccountsApiError::InternalError.to_string() })
        );
    }

    #[actix_web::test]
    async fn test_get_account_not_found_error() {
        let customer_id = 1;
        let account_id = 4;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| Err(RepoError::NotFound));

        let res = get_account(Data::new(mock_repo), (customer_id, account_id).into()).await;

        assert!(res.is_err_and(|e| { e.to_string() == AccountsApiError::NotFound.to_string() }));
    }

    #[actix_web::test]
    async fn test_get_account_unauthorized_error() {
        let customer_id = 1;
        let wrong_customer_id = 5;
        let account_id = 4;

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance_cents: 13424234234,
                    account_type: AccountType::Savings,
                    date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap(),
                    account_status: AccountStatus::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                })
            });

        let res = get_account(Data::new(mock_repo), (wrong_customer_id, account_id).into()).await;

        assert!(res.is_err_and(|e| { e.to_string() == AccountsApiError::Unauthorized.to_string() }));
    }

    #[actix_web::test]
    async fn test_delete_account_success() {
        let customer_id = 1;
        let account_id = 2;

        let mut seq = Sequence::new();

        let mut mock_repo = MockAccountsRepository::new();

        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance_cents: 13424234234,
                    account_type: AccountType::Savings,
                    date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap(),
                    account_status: AccountStatus::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                })
            })
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

        let mut mock_repo = MockAccountsRepository::new();
        mock_repo
            .expect_get_account()
            .with(eq(account_id))
            .times(1)
            .returning(move |_| {
                Ok(Account {
                    id: account_id,
                    customer_id,
                    balance_cents: 13424234234,
                    account_type: AccountType::Savings,
                    date_opened: NaiveDate::from_ymd_opt(2016, 7, 8)
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap(),
                    account_status: AccountStatus::Active,
                    name: Some("abc".to_string()),
                    available_balance_cents: 34343,
                    account_number: "012345678".to_string(),
                    bsb: 123456,
                })
            });

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
