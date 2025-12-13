use actix_web::{HttpResponse, Responder, delete, get, post, web};
use actix_web_validator::{Json, Query};
use bcrypt::{DEFAULT_COST, hash};

use crate::{
    models::{CreateUserInput, ListUsersInput, Page, UserModel},
    state::AppState,
    utils::{name::valid_name, password::is_secure_password},
};

#[utoipa::path()]
#[post("/api/users")]
async fn create(payload: Json<CreateUserInput>, state: web::Data<AppState>) -> impl Responder {
    let payload = payload.into_inner();

    if !valid_name(&payload.first_name) {
        return HttpResponse::BadRequest().finish();
    }
    if !valid_name(&payload.last_name) {
        return HttpResponse::BadRequest().finish();
    }
    if !is_secure_password(&payload.password) {
        return HttpResponse::BadRequest().finish();
    }

    let query = state.users.get_by_email(payload.email.clone());
    let exists = match query.await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if exists {
        return HttpResponse::Conflict().finish();
    }

    let hashed = match hash(&payload.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let query = state
        .users
        .create(payload.email, hashed, payload.first_name, payload.last_name);

    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = UserModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/users")]
async fn list(query: Query<ListUsersInput>, state: web::Data<AppState>) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.users.list(limit, offset);
    let result = match query.await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let models = result.data.iter().map(UserModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/users/{id}")]
async fn get(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let query = state.users.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = UserModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[delete("/api/users/{id}")]
async fn delete(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let query = state.users.delete(id);
    match query.await {
        Ok(result) => {
            if !result {
                return HttpResponse::NotFound().finish();
            }
            return HttpResponse::NoContent().finish();
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
}
