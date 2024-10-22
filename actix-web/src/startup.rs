//! src/startup.rs
use crate::routes::{health_checking, subscribe, AppState};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// #[actix_web::main]

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checking))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Zero2Prod"),
            }))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
