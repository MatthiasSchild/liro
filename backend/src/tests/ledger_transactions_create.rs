use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr, prelude::Date};
use serde_json::json;

use crate::{
    handlers,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_create()
        .once()
        .with(
            predicate::eq(1),
            predicate::eq(2),
            predicate::eq("2025-07-01".to_string()),
            predicate::eq(1337),
        )
        .returning(|debit_account_id, credit_account_id, date, amount| {
            let date = Date::parse_from_str(&date, "%Y-%m-%d").unwrap();

            Ok(entity::ledger_transactions::Model {
                id: 1,
                debit_account_id,
                credit_account_id,
                date: date,
                amount,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": 1,
            "creditAccountId": 2,
            "date": "2025-07-01",
            "amount": 1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_empty() {
    let mut state = MockAppState::new();
    state.ledger_transactions.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_account() {
    let mut state = MockAppState::new();
    state.ledger_transactions.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": -5,
            "creditAccountId": 2,
            "date": "2025-07-01",
            "amount": 1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": 1,
            "creditAccountId": -5,
            "date": "2025-07-01",
            "amount": 1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_date() {
    let mut state = MockAppState::new();
    state.ledger_transactions.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": 1,
            "creditAccountId": 2,
            "date": "invalid",
            "amount": 1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_amount() {
    let mut state = MockAppState::new();
    state.ledger_transactions.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": 1,
            "creditAccountId": 2,
            "date": "2025-07-01",
            "amount": -1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .ledger_transactions
        .expect_create()
        .once()
        .returning(|_, _, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_transactions::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-transactions")
        .set_json(json!({
            "debitAccountId": 1,
            "creditAccountId": 2,
            "date": "2025-07-01",
            "amount": 1337,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
