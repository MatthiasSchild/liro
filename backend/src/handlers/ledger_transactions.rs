use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::{Json, Query};
use regex::Regex;

use crate::{
    models::{
        CreateLedgerTransactionInput, LedgerTransactionModel, ListLedgerTransactionsInput, Page,
    },
    state::AppState,
};

#[utoipa::path()]
#[post("/api/ledger-transactions")]
async fn create(
    payload: Json<CreateLedgerTransactionInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if payload.debit_account_id <= 0 || payload.credit_account_id <= 0 {
        return HttpResponse::BadRequest().finish();
    }

    if payload.amount <= 0 {
        return HttpResponse::BadRequest().finish();
    }

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !re.is_match(&payload.date) {
        return HttpResponse::BadRequest().finish();
    }

    let query = state.ledger_transactions.create(
        payload.debit_account_id,
        payload.credit_account_id,
        payload.date.clone(),
        payload.amount,
    );

    match query.await {
        Ok(entity) => {
            let model = LedgerTransactionModel::from(&entity);
            HttpResponse::Ok().json(model)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path()]
#[get("/api/ledger-transactions")]
async fn list(
    query: Query<ListLedgerTransactionsInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let account_id = query.account_id;
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.ledger_transactions.list(account_id, limit, offset);

    let result = match query.await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let models = result
        .data
        .iter()
        .map(LedgerTransactionModel::from)
        .collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/ledger-transactions/{id}")]
async fn get(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.ledger_transactions.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = LedgerTransactionModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[delete("/api/ledger-transactions/{id}")]
async fn delete(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.ledger_transactions.delete(id);
    let found = match query.await {
        Ok(success) => success,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if !found {
        return HttpResponse::NotFound().finish();
    }

    return HttpResponse::NoContent().finish();
}
