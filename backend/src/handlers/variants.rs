use actix_web::{HttpResponse, delete, get, post, web};
use actix_web_validator::{Json, Path};

use crate::{
    err::{self, ApiErrors},
    models::{CreateVariantInput, ListVariantsInput, Page, VariantModel, VariantsPath},
    state::AppState,
};

#[utoipa::path()]
#[post("/api/variants")]
async fn create(payload: Json<CreateVariantInput>, state: web::Data<AppState>) -> HttpResponse {
    let payload = payload.into_inner();

    let query = state.products.get(payload.product_id);
    let product_exists = match query.await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !product_exists {
        return HttpResponse::NotFound().finish();
    }

    let query = state.variants.create(
        payload.product_id,
        payload.name,
        payload.sale_price,
        payload.purchase_price,
    );
    let entity = match query.await {
        Ok(entity) => entity,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let model = VariantModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path()]
#[get("/api/products/{id}/variants")]
async fn list(
    path: web::Path<i32>,
    query: web::Query<ListVariantsInput>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let product_id = path.into_inner();
    let limit = query.offset.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);

    let query = state.products.get(product_id);
    let product_exists = match query.await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if !product_exists {
        return HttpResponse::NotFound().finish();
    }

    let query = state.variants.list(product_id, limit, offset);
    let result = match query.await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let models = result.data.iter().map(VariantModel::from).collect();
    HttpResponse::Ok().json(Page {
        limit,
        offset,
        total: result.total,
        data: models,
    })
}

#[utoipa::path()]
#[get("/api/variants/{id}")]
async fn get(path: Path<VariantsPath>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner().id;

    let query = state.variants.get(id);
    let entity = match query.await {
        Ok(Some(entity)) => entity,
        Ok(None) => return ApiErrors::VariantNotFound.into(),
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    let model = VariantModel::from(&entity);
    HttpResponse::Ok().json(model)
}

#[utoipa::path(
    tag = "Variants",
    summary = "Delete a variant",
    description = "Delete an existing variant",
    responses(
        (status = 204, description = "Variant has been deleted"),
        (status = 400, description  = err::MESSAGE_VARIANT_NOT_FOUND, content(
            (String = "application/json", example = json!({
                "error": err::MESSAGE_VARIANT_NOT_FOUND,
                "errorCode": err::CODE_VARIANT_NOT_FOUND,
            }))
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
#[delete("/api/variants/{id}")]
async fn delete(path: Path<VariantsPath>, state: web::Data<AppState>) -> HttpResponse {
    let id = path.into_inner().id;

    let query = state.variants.delete(id);
    let found = match query.await {
        Ok(found) => found,
        Err(_) => return ApiErrors::InternalServerError.into(),
    };

    if !found {
        return ApiErrors::VariantNotFound.into();
    }

    HttpResponse::NoContent().finish()
}
