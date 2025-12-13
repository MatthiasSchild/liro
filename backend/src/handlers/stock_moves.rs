use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/stock-moves")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/stock-moves")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/stock-moves/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/stock-moves/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
