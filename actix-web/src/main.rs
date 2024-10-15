// !main.rs
#[warn(unused_imports)]
use sqlx::{Connection, PgConnection};

use std::net::TcpListener;
use zero2prod::{
    configuration::{self, configuration_get},
    startup::run,
};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let configuration = configuration_get().unwrap();
    //TODO: ADd connection
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    //let port = listener.local_addr().unwrap().port();
    //let server = zero2prod::run(listener).expect("Unable to run the http server");
    let port = listener.local_addr().unwrap().port();
    println!("Run program at port {port}");
    run(listener)?.await
}
