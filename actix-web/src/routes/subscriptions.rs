// use actix_web::dev::Server;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
//use log::{debug, error, info, trace, warn};
use sqlx::PgPool;
//use tracing::{debug, error, info};
use uuid::Uuid;
// use std::net::TcpListener;
use tracing::Instrument;
pub struct AppState {
    pub app_name: String,
}

#[derive(serde::Deserialize)]
//#[warn(unused]

pub struct User {
    email: String,
    name: String,
}

pub async fn health_checking(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let app_data = &data.app_name;
    println!("Run in to health_checking - the app_state {}", app_data);
    HttpResponse::Ok().finish()
}
/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
pub async fn subscribe(form: web::Form<User>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let span = tracing::span!(tracing::Level::WARN, "Adding new subcriber", Request_id = %request_id, subcriber_username = %form.name, subcriber_email = %form.email);
    // tracing::info!(
    //     "Request id {} - Saving new subscriber '{}' '{}' in the database",
    //     request_id,
    //     form.name,
    //     form.email
    // );
    let _subcriber_guard = span.enter();
    tracing::info!(
        "Request id {} - Saving new subcribers in the database",
        request_id
    );
    let query_span = tracing::span!(tracing::Level::INFO, "Saving new subscriber to database");
    let _guard_query_span = query_span.enter();
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .in_current_span()
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Request id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "Request id {} - Error when insert user data for subscription {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
