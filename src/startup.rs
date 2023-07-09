use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, Error> {
        // Acquire Database connection
        let connction_ppol = get_connection_pool(&configuration.database);

        // Timeout for email client
        let timeout = std::time::Duration::from_millis(configuration.email_client.timeout_milli);
        // Sender
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid Sender email");

        let email_client = EmailClient::new(
            sender_email,
            configuration.email_client.sender_api_url,
            configuration.email_client.authorization_token,
            timeout,
        );

        // let address = format!("127.0.0.1:{}", configuration.application_port);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connction_ppol,
            email_client,
            configuration.application.base_url,
        )?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(pool);
    let email_client = web::Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
