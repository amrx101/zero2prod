//! src/routes/subscriptions_confirm.rs

use actix_web::{web, HttpResponse};

#[derive(Debug)]
pub struct Parameters {
    subscription_token: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name = "COnfirm a pending subscriber")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}
