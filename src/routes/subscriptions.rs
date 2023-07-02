use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse{
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} = Addind '{}' '{}' as a subscriber.",
        request_id,
        form.email,
        form.name,
    );
    tracing::info!("request _id {} - Saving new subscriber details in the database", request_id);

    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subsriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );

    let res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#, Uuid::new_v4(), form.email, form.name, Utc::now()
    )
        .execute(pool.get_ref())
        .instrument(query_span)
        .await;
    match res {
        Ok(_) =>{
            tracing::info!(
                "request_id {} - New subscriber details have been saved", request_id
            );
            // log::info!("New subscriber details have been saved!.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            // log::error!("Failed to execute query : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

