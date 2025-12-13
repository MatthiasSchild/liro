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
        .users
        .expect_get_by_email()
        .once()
        .with(predicate::eq("user@example.org".to_string()))
        .returning(|_| Ok(None));
    state
        .users
        .expect_create()
        .once()
        .with(
            predicate::eq("user@example.org".to_string()),
            predicate::always(),
            predicate::eq("Admin".to_string()),
            predicate::eq("Example".to_string()),
        )
        .returning(|email, pass_hash, first_name, last_name| {
            Ok(entity::users::Model {
                id: 1,
                email,
                pass_hash,
                first_name,
                last_name,
            })
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "Password123!",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    // TODO Check, if password is correctly hashed
}

#[actix_web::test]
async fn test_email_already_exists() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_get_by_email()
        .once()
        .with(predicate::eq("user@example.org".to_string()))
        .returning(|email| {
            Ok(Some(entity::users::Model {
                id: 1,
                email,
                pass_hash: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
            }))
        });
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "Password123!",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 409u16);
}

#[actix_web::test]
async fn test_insecure_password() {
    let mut state = MockAppState::new();
    state.users.expect_get_by_email().never();
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "a",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_email() {
    let mut state = MockAppState::new();
    state.users.expect_get_by_email().never();
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "email",
            "password": "password",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_first_name() {
    let mut state = MockAppState::new();
    state.users.expect_get_by_email().never();
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "password",
            "firstName": "a",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_invalid_last_name() {
    let mut state = MockAppState::new();
    state.users.expect_get_by_email().never();
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "password",
            "firstName": "Admin",
            "lastName": "a",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 400u16);
}

#[actix_web::test]
async fn test_connection_error1() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_get_by_email()
        .once()
        .with(predicate::eq("user@example.org".to_string()))
        .returning(|_| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });
    state.users.expect_create().never();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "Password123!",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}

#[actix_web::test]
async fn test_connection_error2() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_get_by_email()
        .once()
        .with(predicate::eq("user@example.org".to_string()))
        .returning(|_| Ok(None));
    state
        .users
        .expect_create()
        .once()
        .with(
            predicate::eq("user@example.org".to_string()),
            predicate::always(),
            predicate::eq("Admin".to_string()),
            predicate::eq("Example".to_string()),
        )
        .returning(|_, _, _, _| {
            let err = DbErr::Conn(RuntimeErr::Internal("mocked connection error".to_string()));
            Err(err)
        });

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::from(state)))
            .service(handlers::users::create),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "email": "user@example.org",
            "password": "Password123!",
            "firstName": "Admin",
            "lastName": "Example",
        }))
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
