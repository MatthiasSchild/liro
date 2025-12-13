use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/sale-orders")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-orders")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-orders/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/sale-orders/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
