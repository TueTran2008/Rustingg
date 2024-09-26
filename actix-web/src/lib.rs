use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct User {
    email: String,
    name: String,
}

#[warn(dead_code)]
#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}
#[warn(dead_code)]
#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

async fn health_checking(_req: HttpRequest) -> impl Responder {
    println!("RUn in to health_checking");
    HttpResponse::Ok().finish()
}
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
async fn subcribe(_form: web::Form<User>) -> HttpResponse {
    println!("Run into subcribe");
    HttpResponse::Ok().finish()
}
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// #[actix_web::main]
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checking))
            .route("/subscriptions", web::post().to(subcribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
