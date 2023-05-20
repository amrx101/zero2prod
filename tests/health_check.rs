use std::borrow::Borrow;
use tokio::spawn;
use zero2prod::startup::run;
use std::net::TcpListener;
use actix_web::web::Buf;
use sqlx::{PgConnection, Connection, PgPool, Executor};
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str()) .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string()) .await
        .expect("Failed to connect to Postgres."); sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}


async fn spawn_app() -> TestApp{
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port(); let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}


// #[tokio::test]
// async fn health_check_works() {
//     let address = spawn_app();
//
//     let client = reqwest::Client::new();
//
//     let response = client
//         .get(&format!("{}/health_check", address))
//         .send()
//         .await
//         .expect("Failed to execute request.");
//
//     assert!(response.status().is_success());
//     assert_eq!(Some(0), response.content_length());
// }

// fn spawn_app() -> String {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind.");
//     let port = listener.local_addr().unwrap().port();
//     let server = run(listener).expect("Faild to spawn server.");
//
//     let _ = tokio::spawn(server);
//     format!("http://127.0.0.1:{}", port)
// }

// #[tokio::test]
// async fn subscribe_returns_200() {
//     let address = spawn_app();
//
//     let client = reqwest::Client::new();
//
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//
//     let response = client
//         .post(&format!("{}/subscriptions", address))
//         .header("Content-Type", "application/x-www-form-urlencode")
//         .body(body)
//         .send()
//         .await
//         .expect("Falied to execute request");
//
//     assert_eq!(200, response.status().as_u16());
//
// }

// #[tokio::test]
// async fn subscribe_returns_404() {
//
//     let address = spawn_app();
//     let client = reqwest::Client::new();
//     let test_cases = vec![
//         ("name=le%20guin", "missing the email"),
//         ("email=ursula_le_guin%40gmail.com", "missing the name"),
//         ("", "missing both name and email")
//     ];
//
//
//     for (invalid_body, error_message) in test_cases {
//         // Act
//         let response = client
//             .post(&format!("{}/subscriptions", &address)).header("Content-Type", "application/x-www-form-urlencoded").body(invalid_body)
//             .send()
//             .await
//             .expect("Failed to execute request.");
// // Assert
//         assert_eq!(400,
//                    response.status().as_u16(),
//                    // Additional customised error message on test failure
//                    "The API did not fail with 400 Bad Request when the payload was {}.",
//                    error_message
//         );
//     }
// }

#[tokio::test]
async fn subscribe_returns_200_for_valid_froms(){

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address)) .header("Content-Type", "application/x-www-form-urlencoded") .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
// Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to save subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");

}