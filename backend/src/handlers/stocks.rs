use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::Json;

use crate::{
    models::{CreateStockInput, StockModel},
    state::AppState,
};

#[utoipa::path()]
#[post("/api/stocks")]
pub async fn create_own(
    payload: Json<CreateStockInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let payload = payload.into_inner();
    let query = state.stocks.create_own(payload.name);

    match query.await {
        Ok(entity) => {
            let model = StockModel::from(&entity);
            HttpResponse::Ok().json(model)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path()]
#[post("/api/contacts/{contact}/stocks")]
pub async fn create_contact(
    path: web::Path<i32>,
    payload: Json<CreateStockInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let contact = path.into_inner();
    let payload = payload.into_inner();

    let query = state.contacts.get(contact);
    let contact = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let query = state.stocks.create_for_contact(payload.name, contact.id);
    let stock = match query.await {
        Ok(entity) => entity,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = StockModel::from(&stock);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/stocks")]
pub async fn list_own() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/contacts/{contact}/stocks")]
pub async fn list_contract() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/stocks/{id}")]
pub async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/stocks/{id}")]
pub async fn delete() -> HttpResponse {
    todo!();
}
