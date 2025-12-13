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
        .users
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|id| {
            Ok(Some(entity::users::Model {
                id,
                email: "erp@example.org".to_string(),
                pass_hash: "".to_string(),
                first_name: "erp".to_string(),
                last_name: "admin".to_string(),
            }))
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    // Never leak anything like a password
    let payload: serde_json::Value = test::read_body_json(res).await;
    if let Some(obj) = payload.as_object() {
        let has_pass = obj.keys().any(|key| key.to_lowercase().contains("pass"));
        assert_eq!(has_pass, false);
    }
}

#[actix_web::test]
async fn test_not_found() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_get()
        .once()
        .with(predicate::eq(5))
        .returning(|_| Ok(None));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 404u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .users
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
            .service(handlers::users::get),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users/5").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
