use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

use crate::{
    handlers,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_customer_success() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::contacts::Model {
                id,
                name: "Example Inc.".to_string(),
                is_customer: true,
                is_supplier: false,
                street1: "street1".to_string(),
                street2: "street2".to_string(),
                postal_code: "postalCode".to_string(),
                city: "city".to_string(),
                country: "country".to_string(),
                customer_account_id: Some(5),
                supplier_account_id: None,
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_customer),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["id"], 5);
    assert_eq!(payload["name"], "Example Inc.".to_string());
    assert_eq!(payload["isCustomer"], true);
    assert_eq!(payload["isSupplier"], false);
    assert_eq!(payload["customerAccountId"], 5);
    assert_eq!(payload["supplierAccountId"], serde_json::Value::Null);
    assert_eq!(payload["street1"], "street1".to_string());
    assert_eq!(payload["street2"], "street2".to_string());
    assert_eq!(payload["postalCode"], "postalCode".to_string());
    assert_eq!(payload["city"], "city".to_string());
    assert_eq!(payload["country"], "country".to_string());
}

#[actix_web::test]
async fn test_supplier_success() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::contacts::Model {
                id,
                name: "Example Inc.".to_string(),
                is_customer: false,
                is_supplier: true,
                street1: "street1".to_string(),
                street2: "street2".to_string(),
                postal_code: "postalCode".to_string(),
                city: "city".to_string(),
                country: "country".to_string(),
                customer_account_id: None,
                supplier_account_id: Some(5),
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_supplier),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/suppliers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["id"], 5);
    assert_eq!(payload["name"], "Example Inc.".to_string());
    assert_eq!(payload["isCustomer"], false);
    assert_eq!(payload["isSupplier"], true);
    assert_eq!(payload["customerAccountId"], serde_json::Value::Null);
    assert_eq!(payload["supplierAccountId"], 5);
    assert_eq!(payload["street1"], "street1".to_string());
    assert_eq!(payload["street2"], "street2".to_string());
    assert_eq!(payload["postalCode"], "postalCode".to_string());
    assert_eq!(payload["city"], "city".to_string());
    assert_eq!(payload["country"], "country".to_string());
}

#[actix_web::test]
async fn test_customer_not_found1() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_customer),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_customer_not_found2() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::contacts::Model {
                id,
                name: "Example Inc.".to_string(),
                is_customer: false,
                is_supplier: true,
                street1: "street1".to_string(),
                street2: "street2".to_string(),
                postal_code: "postalCode".to_string(),
                city: "city".to_string(),
                country: "country".to_string(),
                customer_account_id: None,
                supplier_account_id: Some(5),
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_customer),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_supplier_not_found1() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_supplier),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/suppliers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_supplier_not_found2() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::contacts::Model {
                id,
                name: "Example Inc.".to_string(),
                is_customer: true,
                is_supplier: false,
                street1: "street1".to_string(),
                street2: "street2".to_string(),
                postal_code: "postalCode".to_string(),
                city: "city".to_string(),
                country: "country".to_string(),
                customer_account_id: Some(5),
                supplier_account_id: None,
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_supplier),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/suppliers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::get_customer),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
