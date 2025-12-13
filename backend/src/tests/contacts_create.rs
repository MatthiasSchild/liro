use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};
use serde_json::json;

use crate::{
    handlers,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_customer_success() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::ledger_accounts::Model {
                id,
                account_type: "income".to_string(),
                name: "Example Inc.".to_string(),
            }))
        });
    state
        .contacts
        .expect_create_customer()
        .once()
        .with(
            predicate::eq("Example Inc.".to_string()),
            predicate::eq(5),
            predicate::eq("street1".to_string()),
            predicate::eq("street2".to_string()),
            predicate::eq("postalCode".to_string()),
            predicate::eq("city".to_string()),
            predicate::eq("country".to_string()),
        )
        .returning(
            |name, account_id, street1, street2, postal_code, city, country| {
                Ok(entity::contacts::Model {
                    id: 1,
                    name,
                    is_customer: true,
                    is_supplier: false,
                    street1,
                    street2,
                    postal_code,
                    city,
                    country,
                    customer_account_id: Some(account_id),
                    supplier_account_id: None,
                })
            },
        );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["id"], 1);
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
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::ledger_accounts::Model {
                id,
                account_type: "expense".to_string(),
                name: "Example Inc.".to_string(),
            }))
        });
    state
        .contacts
        .expect_create_supplier()
        .once()
        .with(
            predicate::eq("Example Inc.".to_string()),
            predicate::eq(5),
            predicate::eq("street1".to_string()),
            predicate::eq("street2".to_string()),
            predicate::eq("postalCode".to_string()),
            predicate::eq("city".to_string()),
            predicate::eq("country".to_string()),
        )
        .returning(
            |name, account_id, street1, street2, postal_code, city, country| {
                Ok(entity::contacts::Model {
                    id: 1,
                    name,
                    is_customer: false,
                    is_supplier: true,
                    street1,
                    street2,
                    postal_code,
                    city,
                    country,
                    customer_account_id: None,
                    supplier_account_id: Some(account_id),
                })
            },
        );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_supplier),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/suppliers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    let payload: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(payload["id"], 1);
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
async fn test_customer_invalid_name() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.contacts.expect_create_customer().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "",
            "accountId": 5,
            "street1": "",
            "street2": "",
            "postalCode": "",
            "city": "",
            "country": "",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_supplier_invalid_name() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.contacts.expect_create_supplier().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_supplier),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/suppliers")
        .set_json(json!({
            "name": "",
            "accountId": 5,
            "street1": "",
            "street2": "",
            "postalCode": "",
            "city": "",
            "country": "",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_customer_invalid_account() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.contacts.expect_create_customer().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": -5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_supplier_invalid_account() {
    let mut state = MockAppState::new();
    state.ledger_accounts.expect_get().never();
    state.contacts.expect_create_supplier().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_supplier),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/suppliers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": -5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_customer_account_not_found() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.contacts.expect_create_customer().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_supplier_account_not_found() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.contacts.expect_create_supplier().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_supplier),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/suppliers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error1() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });
    state.contacts.expect_create_customer().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}

#[actix_web::test]
async fn test_connection_error2() {
    let mut state = MockAppState::new();
    state
        .ledger_accounts
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::ledger_accounts::Model {
                id,
                account_type: "income".to_string(),
                name: "Example Inc.".to_string(),
            }))
        });
    state
        .contacts
        .expect_create_customer()
        .once()
        .with(
            predicate::eq("Example Inc.".to_string()),
            predicate::eq(5),
            predicate::eq("street1".to_string()),
            predicate::eq("street2".to_string()),
            predicate::eq("postalCode".to_string()),
            predicate::eq("city".to_string()),
            predicate::eq("country".to_string()),
        )
        .returning(|_, _, _, _, _, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::create_customer),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/customers")
        .set_json(json!({
            "name": "Example Inc.",
            "accountId": 5,
            "street1": "street1",
            "street2": "street2",
            "postalCode": "postalCode",
            "city": "city",
            "country": "country",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
