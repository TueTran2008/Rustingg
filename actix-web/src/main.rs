// !main.rs
use log;
use sqlx::PgPool;
use std::{default, net::TcpListener};
use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::filter::{Directive, EnvFilter};
use tracing_subscriber::{layer::SubscriberExt, Registry};
use zero2prod::{configuration::configuration_get, startup::run};

fn get_subscriber(name: String, directive: Directive) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::from_default_env().add_directive(directive);
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let global_subscriber = Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(JsonStorageLayer);
    return global_subscriber;
}

fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Get log tracer logger failed");
    //let Subscriber = get_subscriber(String::from("Darwin backend"), LevelFilter::INFO).await;
    set_global_default(subscriber).expect("Failed to set subscriber");
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_subcriber = get_subscriber("darwin".into(), LevelFilter::INFO.into());
    init_subscriber(web_subcriber);
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
