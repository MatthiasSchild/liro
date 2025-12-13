use actix_web::{HttpResponse, get};

#[get("/docs")]
pub async fn redirect() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/docs/"))
        .finish()
}
