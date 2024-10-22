//! src/startup.rs
use crate::routes::{health_checking, subscribe, AppState};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// #[actix_web::main]

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_checking))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Zero2Prod"),
            }))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
