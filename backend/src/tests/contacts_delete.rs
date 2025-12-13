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
        .expect_delete()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(true));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::delete_customer),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 204u16);
}

#[actix_web::test]
async fn test_supplier_success() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_delete()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(true));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::delete_supplier),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/suppliers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 204u16);
}

#[actix_web::test]
async fn test_customer_not_found() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_delete()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(false));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::delete_customer),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_supplier_not_found() {
    let mut state = MockAppState::new();
    state
        .contacts
        .expect_delete()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(false));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::delete_supplier),
    )
    .await;

    let req = test::TestRequest::delete()
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
        .expect_delete()
        .once()
        .with(predicate::eq(5))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::contacts::delete_customer),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/customers/5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
