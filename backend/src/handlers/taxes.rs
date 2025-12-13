use actix_web::{HttpResponse, delete, get, post};

#[utoipa::path()]
#[post("/api/taxes")]
async fn create() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/taxes")]
async fn list() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[get("/api/taxes/{id}")]
async fn get() -> HttpResponse {
    todo!();
}

#[utoipa::path()]
#[delete("/api/taxes/{id}")]
async fn delete() -> HttpResponse {
    todo!();
}
