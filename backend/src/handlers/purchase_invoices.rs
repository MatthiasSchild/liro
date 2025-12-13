use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/purchase-invoices")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-invoices")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/purchase-invoices/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/purchase-invoices/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
