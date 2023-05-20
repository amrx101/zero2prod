use std::borrow::Borrow;
use tokio::spawn;
use zero2prod::run;
use std::net::TcpListener;


#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Faild to spawn server.");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencode")
        .body(body)
        .send()
        .await
        .expect("Falied to execute request");

    assert_eq!(200, response.status().as_u16());

}

#[tokio::test]
async fn subscribe_returns_404() {

    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];


    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &address)).header("Content-Type", "application/x-www-form-urlencoded").body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
// Assert
        assert_eq!(400,
                   response.status().as_u16(),
                   // Additional customised error message on test failure
                   "The API did not fail with 400 Bad Request when the payload was {}.",
                   error_message
        );
    }
}