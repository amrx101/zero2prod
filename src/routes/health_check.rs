use actix_web::{HttpResponse, HttpRequest};


pub async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}