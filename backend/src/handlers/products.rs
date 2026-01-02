use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::{Json, Query};

use crate::{
    err::ApiErrors,
    models::{CreateProductInput, ListProductsInput, Page, ProductModel},
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
pub async fn list(query: Query<ListProductsInput>, state: web::Data<AppState>) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.products.list(limit, offset);
    let result = match query.await {
        Ok(result) => result,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let models = result.data.iter().map(ProductModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/products/{id}")]
pub async fn get(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.products.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = ProductModel::from(&entity);
    HttpResponse::Ok().json(model)
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
