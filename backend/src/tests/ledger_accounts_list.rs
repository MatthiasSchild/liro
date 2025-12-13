use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

use crate::{
    handlers,
    models::Page,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success1() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_list()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let entities: Vec<entity::ledger_accounts::Model> = vec![
                entity::ledger_accounts::Model {
                    id: 1,
                    account_type: "asset".to_string(),
                    name: "Bank".to_string(),
                },
                entity::ledger_accounts::Model {
                    id: 2,
                    account_type: "asset".to_string(),
                    name: "Other".to_string(),
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
            .service(handlers::ledger_accounts::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-accounts")
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
        .ledger_accounts
        .expect_list()
        .once()
        .with(predicate::eq(13), predicate::eq(3))
        .returning(|limit, offset| {
            let entities: Vec<entity::ledger_accounts::Model> = vec![
                entity::ledger_accounts::Model {
                    id: 1,
                    account_type: "asset".to_string(),
                    name: "Bank".to_string(),
                },
                entity::ledger_accounts::Model {
                    id: 2,
                    account_type: "asset".to_string(),
                    name: "Other".to_string(),
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
            .service(handlers::ledger_accounts::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-accounts?limit=13&offset=3")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["total"], 2);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_list()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|_, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::ledger_accounts::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/ledger-accounts")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
