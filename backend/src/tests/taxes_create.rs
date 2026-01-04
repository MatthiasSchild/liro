use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};
use serde_json::json;

use crate::{
    handlers,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::ledger_accounts::Model {
                id,
                account_type: "expenses".to_string(),
                name: "Taxes".to_string(),
            }))
        });
    state
        .taxes
        .expect_create()
        .once()
        .with(
            predicate::eq("Sales tax".to_string()),
            predicate::eq("19%".to_string()),
            predicate::eq(0.19),
            predicate::eq(5),
        )
        .returning(|name, name_short, rate, account_id| {
            Ok(entity::taxes::Model {
                id: 1,
                name,
                name_short,
                rate,
                account_id,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "Sales tax",
            "nameShort": "19%",
            "rate": 0.19,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_customer_invalid_name() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.taxes.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "",
            "nameShort": "19%",
            "rate": 0.19,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_supplier_invalid_short_name() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.taxes.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "Sales tax",
            "nameShort": "",
            "rate": 0.19,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_customer_invalid_rate() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.taxes.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "Sales tax",
            "nameShort": "19%",
            "rate": -0.5,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_supplier_invalid_account() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.taxes.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "Sales tax",
            "nameShort": "19%",
            "rate": 0.19,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::ledger_accounts::Model {
                id,
                account_type: "expenses".to_string(),
                name: "Taxes".to_string(),
            }))
        });
    state
        .taxes
        .expect_create()
        .once()
        .with(
            predicate::eq("Sales tax".to_string()),
            predicate::eq("19%".to_string()),
            predicate::eq(0.19),
            predicate::eq(5),
        )
        .returning(|_, _, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::taxes::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/taxes")
        .set_json(json!({
            "name": "Sales tax",
            "nameShort": "19%",
            "rate": 0.19,
            "account": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
