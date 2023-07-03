use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;
use actix_web::error::HttpError;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}



#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form, pool),
    fields(
        subsciber_email=%form.email,
        subscriber_name=%form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>, ) -> HttpResponse {
    if !is_valid_name(&form.name){
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!( r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())

}

pub fn is_valid_name(s :&str)-> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 255;
    // let forbidden_chars = ['/', '\\', '?', '<', '>', ',', '{', '}', '[', ']' ];
    let mut forbidden_chars: HashSet<char> = HashSet::new();
    forbidden_chars.insert('/');
    forbidden_chars.insert('\\');
    forbidden_chars.insert('?');
    forbidden_chars.insert('<');
    forbidden_chars.insert('>');
    forbidden_chars.insert('<');
    let has_forbidden_chars = s.chars().any(|g| forbidden_chars.contains(&g));
    !(is_empty_or_whitespace || is_too_long || has_forbidden_chars)
}