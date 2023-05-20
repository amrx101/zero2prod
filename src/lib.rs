use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;
pub mod configuration;
pub mod routes;
// pub mod startup;


#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name);
    HttpResponse::Ok()
}


async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}


async fn subscribe(_form: web::Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}