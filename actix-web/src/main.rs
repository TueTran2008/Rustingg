// !main.rs
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, Registry};
use zero2prod::{configuration::configuration_get, startup::run};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = configuration_get().unwrap();
    //let connection = PgConnection::connect(&configuration.database.connect_string
    let connection_pool = PgPool::connect(&configuration.database.connect_string())
        .await
        .expect("Get PgPool to create connection to database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    //let port = listener.local_addr().unwrap().port();
    //let server = zero2prod::run(listener).expect("Unable to run the http server");
    let port = listener.local_addr().unwrap().port();
    println!("Run program at port {port}");
    run(listener, connection_pool)?.await
}
