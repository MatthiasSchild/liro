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
        .products
        .expect_create()
        .once()
        .with(predicate::eq("Mug".to_string()))
        .returning(|name| Ok(entity::products::Model { id: 1, name }));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/products")
        .set_json(json!({
            "name": "Mug",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_invalid_name() {
    let mut state = MockAppState::new();
    state.products.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/products")
        .set_json(json!({
            "name": "",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_create()
        .once()
        .with(predicate::eq("Mug".to_string()))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/products")
        .set_json(json!({
            "name": "Mug",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
