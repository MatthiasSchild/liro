use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::{Json, Query};

use crate::{
    err::{self, ApiErrors},
    models::{ContactModel, CreateContactInput, ListContactsInput, Page},
    state::AppState,
};

#[utoipa::path(
    tag = "Customers",
    summary = "Create a customer",
    description = "Create a customer, connect it with an existing account and return the new created customer",
    responses(
        (status = 200, description = "Returns the created customer contact", body = ContactModel),
        (status = 400, description = err::MESSAGE_ACCOUNT_NOT_FOUND, content(
            (String = "application/json", example = json!({
                "error": err::MESSAGE_ACCOUNT_NOT_FOUND,
                "errorCode": err::CODE_ACCOUNT_NOT_FOUND,
            })),
        )),
        (status = 500, description = err::MESSAGE_INTERNAL_SERVER_ERROR, content(
            (String = "application/json", example = json!({
                "error": err::MESSAGE_INTERNAL_SERVER_ERROR,
                "errorCode": err::CODE_INTERNAL_SERVER_ERROR,
            })),
        )),
        (status = 500, description = err::MESSAGE_DATABASE_UNREACHABLE, content(
            (String = "application/json", example = json!({
                "error": err::MESSAGE_DATABASE_UNREACHABLE,
                "errorCode": err::CODE_DATABASE_UNREACHABLE,
            })),
        )),
    )
)]
#[post("/api/customers")]
async fn create_customer(
    payload: Json<CreateContactInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let payload = payload.into_inner();

    let query = state.ledger_accounts.get(payload.account_id);

    match query.await {
        Ok(Some(_)) => {}
        Ok(None) => return ApiErrors::AccountNotFound.into(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let query = state.contacts.create_customer(
        payload.name,
        payload.account_id,
        payload.street1,
        payload.street2,
        payload.postal_code,
        payload.city,
        payload.country,
    );

    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = ContactModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path(
    tag = "Suppliers",
    summary = "Create a supplier",
    description = "Create a supplier, connect it with an existing account and return the new created supplier",
    responses(
        (status = 200, description = "Returns the created supplier contact", body = ContactModel),
        (status = 404, description = "Ledger account could not be found")
    )
)]
#[post("/api/suppliers")]
async fn create_supplier(
    payload: Json<CreateContactInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let payload = payload.into_inner();

    let query = state.ledger_accounts.get(payload.account_id);

    match query.await {
        Ok(Some(_)) => {}
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let query = state.contacts.create_supplier(
        payload.name,
        payload.account_id,
        payload.street1,
        payload.street2,
        payload.postal_code,
        payload.city,
        payload.country,
    );

    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = ContactModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/customers")]
async fn list_customers(
    query: Query<ListContactsInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.contacts.list_customers(limit, offset);
    let result = match query.await {
        Ok(result) => result,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let models = result.data.iter().map(ContactModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/suppliers")]
async fn list_suppliers(
    query: Query<ListContactsInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.contacts.list_suppliers(limit, offset);
    let result = match query.await {
        Ok(result) => result,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let models = result.data.iter().map(ContactModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/customers/{id}")]
async fn get_customer(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.contacts.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !entity.is_customer {
        return HttpResponse::NotFound().finish();
    }

    let model = ContactModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/suppliers/{id}")]
async fn get_supplier(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.contacts.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !entity.is_supplier {
        return HttpResponse::NotFound().finish();
    }

    let model = ContactModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[delete("/api/customers/{id}")]
async fn delete_customer(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.contacts.delete(id);
    let found = match query.await {
        Ok(found) => found,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !found {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::NoContent().finish()
}

#[utoipa::path()]
#[delete("/api/suppliers/{id}")]
async fn delete_supplier(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner();

    let query = state.contacts.delete(id);
    let found = match query.await {
        Ok(found) => found,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !found {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::NoContent().finish()
}
