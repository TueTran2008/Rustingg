//! tests/health_check.rs

use crate::configuration::configuration_get;
use crate::telemetry::*;
use crate::{configuration::DatabaseSettings, startup::run};
use sqlx::{Executor, PgPool};
use std::net::TcpListener;
use std::sync::OnceLock;
use tracing_subscriber::filter::LevelFilter;
use uuid::Uuid;
struct TestData {
    address: String,
    pool: PgPool,
}
static TRACING: OnceLock<()> = OnceLock::new();

#[tokio::test]
async fn health_check_works() {
    // let configuration = configuration_get().unwrap();
    // //let connect_string = configuration.database.connect_string();
    // //println!("Connection string {}", &connect_string);
    // let sql_connection = PgConnection::connect(&connect_string)
    //     .await
    //     .expect("Fail to connect to postgres with CMD: {}");
    let app = spawn_app().await;
    //we need `reqwest` to perform HTTP requests to our application
    let client = reqwest::Client::new();
    // Action
    let reponse = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(reponse.status().is_success());
    assert_eq!(Some(0), reponse.content_length());
}
#[tokio::test]
async fn subcribe_return_a_200_for_valid_form_data() {
    //we need `reqwest` to perform HTTP requests to our application
    // let configuration = configuration_get().unwrap();
    // //let connect_string = configuration.database.connect_string();
    // //println!("Connection string {}", &connect_string);
    // let sql_connection = PgConnection::connect(&connect_string)
    //     .await
    //     .expect("Fail to connect to postgres with CMD: {}");
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=Darwin_Tran&email=darwin_tran%40vn.gemteks.com";
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    // assert!(reponse.status().is_success());
    // assert_eq!(Some(0), reponse.content_length());
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(saved.email, "darwin_tran@vn.gemteks.com");
    assert_eq!(saved.name, "Darwin_Tran");
}

#[tokio::test]
async fn subcribe_return_a_400_for_invalid_form_data() {
    let app = spawn_app().await;
    //we need `reqwest` to perform HTTP requests to our application
    let client = reqwest::Client::new();
    let invalid_vec = vec![
        ("name=Darwin_Tran", "missing email"),
        ("email=Darwin_Tran%40vn.gemteks.com", "missing name"),
        ("", "Missing all"),
    ];
    for (invalid_msg, error_msg) in invalid_vec {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_msg)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with bad request 400 when the payload was {}",
            error_msg
        );
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //Create Database
    let connection = PgPool::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect with Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Postgres database");
    let connection_pool = PgPool::connect(&config.connect_string())
        .await
        .expect("Failed to connect with Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate Posgres");
    connection_pool
}
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
//Lauch our application in background  ~somehow~
async fn spawn_app() -> TestData {
    assert!(TRACING.get().is_none());
    let _tracing = TRACING.get_or_init(|| {
        let test_sub = get_subscriber("test_debug".into(), LevelFilter::INFO.into());
        init_subscriber(test_sub);
    });

    assert!(TRACING.get().is_none());
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let mut configuration = configuration_get().unwrap();
    configuration.database.database_name = Uuid::new_v4().to_string();
    // let migrate_str = format!;("{}/migrations", configuration.database.database_name);
    // let migrate = sqlx::migrate!();
    //let connection = PgConnection::connect(&configuration.database.connect_string());
    // let connection_pool = PgPool::connect(&configuration.database.connect_string())
    //     .await
    //     .expect("Get PgPool to create connection to database");

    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone()).expect("Unable to run the http server");
    //let server = zero2prod::run("127.0.0.1:0").expect("Run the http server");
    tokio::spawn(server);

    //format!("http://127.0.0.1:{}", port)
    TestData {
        address: format!("http://127.0.0.1:{}", port),
        pool: connection_pool,
    }
}
