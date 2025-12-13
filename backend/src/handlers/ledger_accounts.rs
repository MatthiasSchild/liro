use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::{Json, Query};

use crate::{
    err::ApiErrors, models::{CreateLedgerAccountInput, LedgerAccountModel, ListLedgerAccountsInput, Page}, state::AppState
};

#[utoipa::path()]
#[post("/api/ledger-accounts")]
async fn create(
    payload: Json<CreateLedgerAccountInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let allowed_types = vec!["asset", "liability", "income", "expense"];
    if !allowed_types.contains(&payload.account_type.as_str()) {
        return ApiErrors::InvalidAccountType.into();
    }

    let query = state
        .ledger_accounts
        .create(payload.account_type.clone(), payload.name.clone());

    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = LedgerAccountModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/ledger-accounts")]
async fn list(query: Query<ListLedgerAccountsInput>, state: web::Data<AppState>) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.ledger_accounts.list(limit, offset);

    let result = match query.await {
        Ok(result) => result,
        Err(err) => {
            tracing::error!("An error occurred: {err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let models = result.data.iter().map(LedgerAccountModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/ledger-accounts/{id}")]
async fn get(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.ledger_accounts.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = LedgerAccountModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[delete("/api/ledger-accounts/{id}")]
async fn delete(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.ledger_accounts.delete(id);
    let found = match query.await {
        Ok(success) => success,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if !found {
        return HttpResponse::NotFound().finish();
    }

    return HttpResponse::NoContent().finish();
}
