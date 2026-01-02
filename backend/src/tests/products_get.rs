use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

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
                name: "Dummy product".to_string(),
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_not_found() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error() {
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
