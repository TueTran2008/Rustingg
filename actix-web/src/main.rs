// !main.rs
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::level_filters::LevelFilter;

use zero2prod::{configuration::configuration_get, startup::run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_subcriber = telemetry::get_subscriber("zero2prod_app".into(), LevelFilter::INFO.into());
    telemetry::init_subscriber(web_subcriber);
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
