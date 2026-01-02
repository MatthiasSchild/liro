use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

use crate::{
    handlers,
    models::Page,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_list()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let entities: Vec<entity::products::Model> = vec![];
            Ok(Page {
                limit,
                offset,
                total: entities.len() as u64,
                data: entities,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::list),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_success_with_limit_and_offset() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_list()
        .once()
        .with(predicate::eq(50), predicate::eq(5))
        .returning(|limit, offset| {
            let entities: Vec<entity::products::Model> = vec![];
            Ok(Page {
                limit,
                offset,
                total: entities.len() as u64,
                data: entities,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::products::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products?limit=50&offset=5")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .products
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
            .service(handlers::products::list),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/products").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
