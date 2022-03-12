use actix_web::{get, web, Responder};

#[get("/projects")]
pub async fn all_project() -> impl Responder {
    web::Json("")
}
