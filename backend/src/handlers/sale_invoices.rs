use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/sale-invoices")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-invoices")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/sale-invoices/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/sale-invoices/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
