//! tests/health_check.rs

use std::net::TcpListener;
//use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    //we need `reqwest` to perform HTTP requests to our application
    let client = reqwest::Client::new();
    // Action
    let reponse = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(reponse.status().is_success());
    assert_eq!(Some(0), reponse.content_length());
}
#[tokio::test]
async fn subcribe_return_a_200_for_valid_form_data() {
    let address = spawn_app();
    //we need `reqwest` to perform HTTP requests to our application
    let client = reqwest::Client::new();
    let body = "name=Darwin_Tran&email=
   darwin_tran%40vn.gemteks.com";
    let response = client
        .post(format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    // assert!(reponse.status().is_success());
    // assert_eq!(Some(0), reponse.content_length());
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subcribe_return_a_400_for_invalid_form_data() {
    let address = spawn_app();
    //we need `reqwest` to perform HTTP requests to our application
    let client = reqwest::Client::new();
    let invalid_vec = vec![
        ("name=Darwin_Tran", "missing email"),
        ("email=Darwin_Tran%40vn.gemteks.com", "missing name"),
        ("", "Missing all"),
    ];
    for (invalid_msg, error_msg) in invalid_vec {
        let response = client
            .post(format!("{}/subscriptions", &address))
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
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
//Lauch our application in background  ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Unable to run the http server");
    //let server = zero2prod::run("127.0.0.1:0").expect("Run the http server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
