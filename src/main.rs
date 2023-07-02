use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::PgPool;
use secrecy::ExposeSecret;




#[tokio::main]
async fn main() -> Result<(), std::io::Error> {


    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config");
    let pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret()
    ).await.expect("Falied to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, pool)?.await
}