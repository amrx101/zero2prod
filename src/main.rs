use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use std::time;
use sqlx::PgPool;
use secrecy::ExposeSecret;
use zero2prod::email_client::EmailClient;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {


    // Setting up and initialising telemetry tracing.
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // all configurations: Database anad what not.
    let configuration = get_configuration().expect("Failed to read config");

    let sender_email = configuration.email_client.sender().expect(
        "Invalid Sender Email"
    );

    let timeout = std::time::Duration::from_millis(configuration.email_client.timeout_milli);


    let email_client = EmailClient::new(
        sender_email,
        configuration.email_client.sender_api_url,
        configuration.email_client.authorization_token,
        timeout
    );

    // Acquiring a pool of DB connection.
    let pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret()
    ).await.expect("Falied to connect to Postgres");


    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    // Calling webserver with TCP listener and pool of Database connections
    // to take care.
    run(listener, pool)?.await
}