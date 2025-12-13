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
        .expect_create()
        .once()
        .with(
            predicate::eq("asset".to_string()),
            predicate::eq("bank".to_string()),
        )
        .returning(|account_type, name| {
            Ok(entity::ledger_accounts::Model {
                id: 1,
                account_type: account_type,
                name,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "asset",
            "name": "bank",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["id"], 1);
}

#[actix_web::test]
async fn test_empty1() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "",
            "name": "bank",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_empty2() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "asset",
            "name": "",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_empty3() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_type1() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "invalid",
            "name": "bank",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_type2() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "ASSET",
            "name": "bank",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_create()
        .once()
        .with(
            predicate::eq("asset".to_string()),
            predicate::eq("bank".to_string()),
        )
        .returning(|_, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/ledger-accounts")
        .set_json(json!({
            "type": "asset",
            "name": "bank",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
