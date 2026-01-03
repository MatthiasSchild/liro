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
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::products::Model {
                id,
                name: "Product".to_string(),
            }))
        });
    state
        .variants
        .expect_list()
        .once()
        .with(predicate::eq(5), predicate::eq(100), predicate::eq(0))
        .returning(|product_id, limit, offset| {
            let entities: Vec<entity::variants::Model> = vec![
                entity::variants::Model {
                    id: 101,
                    name: "Variant 1".to_string(),
                    sale_price: 1337,
                    purchase_price: 1337,
                    product_id,
                },
                entity::variants::Model {
                    id: 102,
                    name: "Variant 2".to_string(),
                    sale_price: 1337,
                    purchase_price: 1337,
                    product_id,
                },
                entity::variants::Model {
                    id: 103,
                    name: "Variant 3".to_string(),
                    sale_price: 1337,
                    purchase_price: 1337,
                    product_id,
                },
            ];

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
            .service(handlers::variants::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products/5/variants")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_invalid_product_id() {
    let mut state = MockAppState::new();
    state
        .products
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));
    state.variants.expect_list().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products/5/variants")
        .to_request();

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
        .returning(|id| {
            Ok(Some(entity::products::Model {
                id,
                name: "Product".to_string(),
            }))
        });
    state
        .variants
        .expect_list()
        .once()
        .with(predicate::eq(5), predicate::eq(100), predicate::eq(0))
        .returning(|_, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::variants::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/products/5/variants")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
