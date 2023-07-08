//! src/routes/subscriptions_confirm.rs

use actix_web::HttpResponse;

#[tracing::instrument(
    name="COnfirm a pending subscriber"
)]
pub async fn confirm()->HttpResponse {
    HttpResponse::Ok().finish()
}

