use actix_web::{Responder, get};

#[utoipa::path(
    get,
    responses(
        (status = 200, description = "Returns info about the current backend"),
    ),
)]
#[get("/api/info")]
pub async fn get() -> impl Responder {
    "hello"
}
