use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::Json;

use crate::{
    err::ApiErrors,
    models::{CreateProductInput, ProductModel},
    state::AppState,
};

#[utoipa::path()]
#[post("/api/products")]
pub async fn create(payload: Json<CreateProductInput>, state: web::Data<AppState>) -> HttpResponse {
    let payload = payload.into_inner();

    let query = state.products.create(payload.name);

    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = ProductModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/products")]
pub async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/products/{id}")]
pub async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/products/{id}")]
pub async fn delete(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.products.delete(id);
    let found = match query.await {
        Ok(found) => found,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !found {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::NoContent().finish()
}
