// use actix_web::dev::Server;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

// use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct User {
    email: String,
    name: String,
}

pub async fn health_checking(_req: HttpRequest) -> impl Responder {
    println!("Run in to health_checking");
    HttpResponse::Ok().finish()
}
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
pub async fn subcribe(_form: web::Form<User>) -> HttpResponse {
    println!("Run into subcribe");
    HttpResponse::Ok().finish()
}
