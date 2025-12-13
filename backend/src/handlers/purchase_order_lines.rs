use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/purchase-order-lines")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-order-lines")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-order-lines/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/purchase-order-lines/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
