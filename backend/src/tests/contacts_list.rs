use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

use crate::{
    handlers,
    models::Page,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_customer_success1() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_list_customers()
        .once()
        .with(predicate::eq(13), predicate::eq(5))
        .returning(|limit, offset| {
            let models: Vec<entity::contacts::Model> = vec![];

            Ok(Page {
                limit,
                offset,
                total: models.len() as u64,
                data: models,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::list_customers),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers?limit=13&offset=5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_customer_success2() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_list_customers()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let models: Vec<entity::contacts::Model> = vec![];

            Ok(Page {
                limit,
                offset,
                total: models.len() as u64,
                data: models,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::list_customers),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/customers").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_supplier_success1() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_list_suppliers()
        .once()
        .with(predicate::eq(13), predicate::eq(5))
        .returning(|limit, offset| {
            let models: Vec<entity::contacts::Model> = vec![];

            Ok(Page {
                limit,
                offset,
                total: models.len() as u64,
                data: models,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::list_suppliers),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/suppliers?limit=13&offset=5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_supplier_success2() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_list_suppliers()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let models: Vec<entity::contacts::Model> = vec![];

            Ok(Page {
                limit,
                offset,
                total: models.len() as u64,
                data: models,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::list_suppliers),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/suppliers").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_list_customers()
        .once()
        .with(predicate::eq(13), predicate::eq(5))
        .returning(|_, _: u64| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::list_customers),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/customers?limit=13&offset=5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
