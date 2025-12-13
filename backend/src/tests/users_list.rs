use actix_web::{App, test, web};
use mockall::predicate;
use sea_orm::{DbErr, RuntimeErr};

use crate::{
    handlers,
    models::Page,
    state::{AppState, MockAppState},
};

#[actix_web::test]
async fn test_success1() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_list()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let entities: Vec<entity::users::Model> = vec![
                entity::users::Model {
                    id: 1,
                    email: "user1@example.org".to_string(),
                    pass_hash: "".to_string(),
                    first_name: "User1".to_string(),
                    last_name: "Exampel".to_string(),
                },
                entity::users::Model {
                    id: 2,
                    email: "user2@example.org".to_string(),
                    pass_hash: "".to_string(),
                    first_name: "User2".to_string(),
                    last_name: "Exampel".to_string(),
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
            .service(handlers::users::list),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);

    // Never leak anything like a password
    let payload: serde_json::Value = test::read_body_json(res).await;
    if let Some(obj) = payload[0].as_object() {
        let has_pass = obj.keys().any(|key| key.to_lowercase().contains("pass"));
        assert_eq!(has_pass, false);
    }
}

#[actix_web::test]
async fn test_success2() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_list()
        .once()
        .with(predicate::eq(13), predicate::eq(3))
        .returning(|limit, offset| {
            let entities: Vec<entity::users::Model> = vec![
                entity::users::Model {
                    id: 1,
                    email: "user1@example.org".to_string(),
                    pass_hash: "".to_string(),
                    first_name: "User1".to_string(),
                    last_name: "Exampel".to_string(),
                },
                entity::users::Model {
                    id: 2,
                    email: "user2@example.org".to_string(),
                    pass_hash: "".to_string(),
                    first_name: "User2".to_string(),
                    last_name: "Exampel".to_string(),
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
            .service(handlers::users::list),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/users?limit=13&offset=3")
        .to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_none_found() {
    let mut state = MockAppState::new();
    state
        .users
        .expect_list()
        .once()
        .with(predicate::eq(100), predicate::eq(0))
        .returning(|limit, offset| {
            let entities: Vec<entity::users::Model> = vec![];

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
            .service(handlers::users::list),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 200u16);
}

#[actix_web::test]
async fn test_connection_error() {
    let mut state = MockAppState::new();
    state
        .users
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
            .service(handlers::users::list),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/users").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status().as_u16(), 500u16);
}
