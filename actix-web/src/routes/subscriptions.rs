// use actix_web::dev::Server;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

// use std::net::TcpListener;
pub struct AppState {
    pub app_name: String,
}
#[derive(serde::Deserialize)]
//#[warn(unused]

pub struct User {
    email: String,
    name: String,
}

pub async fn health_checking(/*_req: HttpRequest*/ data: web::Data<AppState>,) -> impl Responder {
    let app_data = &data.app_name;
    println!("Run in to health_checking - the app_state {}", app_data);
    HttpResponse::Ok().finish()
}
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
pub async fn subcribe(_form: web::Form<User>) -> HttpResponse {
    println!("Run into subcribe");
    HttpResponse::Ok().finish()
}
