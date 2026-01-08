use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::Json;

use crate::{
    err::ApiErrors,
    models::{CreateTaxInput, TaxModel},
    state::AppState,
};

#[utoipa::path()]
#[post("/api/taxes")]
async fn create(payload: Json<CreateTaxInput>, state: web::Data<AppState>) -> HttpResponse {
    let payload = payload.into_inner();

    let query = state.ledger_accounts.get(payload.account_id);
    let account = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return ApiErrors::AccountNotFound.into(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let query = state
        .taxes
        .create(payload.name, payload.name_short, payload.rate, account.id);
    let tax = match query.await {
        Ok(entity) => entity,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = TaxModel::from(&tax);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/taxes")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/taxes/{id}")]
async fn get(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.taxes.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = TaxModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[delete("/api/taxes/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
