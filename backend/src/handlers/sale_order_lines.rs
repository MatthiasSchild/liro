use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/sale-order-lines")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-order-lines")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-order-lines/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/sale-order-lines/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
