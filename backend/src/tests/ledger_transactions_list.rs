use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr, prelude::Date};

use crate::{
    handlers,
    models::Page,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success1() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_list()
        .once()
        .with(predicate::eq(1), predicate::eq(100), predicate::eq(0))
        .returning(|account_id, limit, offset| {
            let date = Date::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap();

            let entities: Vec<entity::ledger_transactions::Model> = vec![
                entity::ledger_transactions::Model {
                    id: 1,
                    debit_account_id: account_id,
                    credit_account_id: 100,
                    date: date.clone(),
                    amount: 1337,
                },
                entity::ledger_transactions::Model {
                    id: 2,
                    debit_account_id: account_id,
                    credit_account_id: 101,
                    date,
                    amount: 1338,
                },
            ];

            Ok(Page {
                limit: limit,
                offset: offset,
                total: entities.len() as u64,
                data: entities,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-transactions?account=1")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["total"], 2);
}

#[actix_web::test]
async fn test_success2() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_list()
        .once()
        .with(predicate::eq(5), predicate::eq(13), predicate::eq(3))
        .returning(|account_id, limit, offset| {
            let date = Date::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap();

            let entities: Vec<entity::ledger_transactions::Model> = vec![
                entity::ledger_transactions::Model {
                    id: 1,
                    debit_account_id: account_id,
                    credit_account_id: 100,
                    date: date.clone(),
                    amount: 1337,
                },
                entity::ledger_transactions::Model {
                    id: 2,
                    debit_account_id: account_id,
                    credit_account_id: 101,
                    date,
                    amount: 1338,
                },
            ];

            Ok(Page {
                limit: limit,
                offset: offset,
                total: entities.len() as u64,
                data: entities,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-transactions?account=5&limit=13&offset=3")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["total"], 2);
}

#[actix_web::test]
async fn test_account_not_found() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_list()
        .once()
        .with(predicate::eq(1), predicate::eq(100), predicate::eq(0))
        .returning(|_, limit, offset| {
            Ok(Page {
                limit: limit,
                offset: offset,
                total: 0,
                data: vec![],
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-transactions?account=1")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["total"], 0);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_list()
        .once()
        .with(predicate::eq(1), predicate::eq(100), predicate::eq(0))
        .returning(|_, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-transactions?account=1")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
