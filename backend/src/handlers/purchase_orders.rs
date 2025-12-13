use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/purchase-orders")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-orders")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-orders/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/purchase-orders/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
