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
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::products::Model {
                id,
                name: "Mug".to_string(),
            }))
        });
    state
        .variants
        .expect_create()
        .once()
        .with(
            predicate::eq(5),
            predicate::eq("Red Mug".to_string()),
            predicate::eq(1200),
            predicate::eq(1000),
        )
        .returning(|product_id, name, sale_price, purchase_price| {
            Ok(entity::variants::Model {
                id: 15,
                name,
                sale_price,
                purchase_price,
                product_id,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "Red Mug",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_product_id_not_found() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.variants.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "Red Mug",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_invalid_product_id() {
    let mut state = MockAppState::new();
    state.products.expect_get().never();
    state.variants.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "Red Mug",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": -5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_name() {
    let mut state = MockAppState::new();
    state.products.expect_get().never();
    state.variants.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_connection_error1() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::products::Model {
                id,
                name: "Mug".to_string(),
            }))
        });
    state
        .variants
        .expect_create()
        .once()
        .with(
            predicate::eq(5),
            predicate::eq("Red Mug".to_string()),
            predicate::eq(1200),
            predicate::eq(1000),
        )
        .returning(|_, _, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "Red Mug",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}

#[actix_web::test]
async fn test_connection_error2() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });
    state.variants.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/variants")
        .set_json(json!({
            "name": "Red Mug",
            "salePrice": 1200,
            "purchasePrice": 1000,
            "productId": 5,
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
