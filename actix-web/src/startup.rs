//! src/startup.rs
use crate::routes::{health_checking, subcribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
