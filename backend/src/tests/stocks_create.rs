use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};
use serde_json::json;

use crate::{
    handlers,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_own_success() {
    let mut state = MockAppState::new();
    state
        .stocks
        .expect_create_own()
        .once()
        .with(predicate::eq("Warenhaus".to_string()))
        .returning(|name| {
            Ok(entity::stocks::Model {
                id: 1,
                name,
                is_own: true,
                owner_id: None,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::stocks::create_own),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/stocks")
        .set_json(json!({"name": "Warenhaus"}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_other_success() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::contacts::Model {
                id: id,
                name: "".to_string(),
                is_customer: false,
                is_supplier: false,
                street1: "".to_string(),
                street2: "".to_string(),
                postal_code: "".to_string(),
                city: "".to_string(),
                country: "".to_string(),
                customer_account_id: None,
                supplier_account_id: None,
            }))
        });
    state
        .stocks
        .expect_create_for_contact()
        .once()
        .with(predicate::eq("Warenhaus".to_string()), predicate::eq(5))
        .returning(|name, owner_id| {
            Ok(entity::stocks::Model {
                id: 1,
                name,
                is_own: false,
                owner_id: Some(owner_id),
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::stocks::create_contact),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/contacts/5/stocks")
        .set_json(json!({"name": "Warenhaus"}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_invalid_name() {
    let mut state = MockAppState::new();
    state.stocks.expect_create_own().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::stocks::create_own),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/stocks")
        .set_json(json!({"name": ""}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_owner() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.stocks.expect_create_for_contact().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::stocks::create_own),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/contacts/5/stocks")
        .set_json(json!({"name": "Warenhaus"}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .stocks
        .expect_create_own()
        .once()
        .with(predicate::eq("Warenhaus".to_string()))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::stocks::create_own),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/stocks")
        .set_json(json!({"name": "Warenhaus"}))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
