use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct Email(String);

#[derive(serde::Deserialize)]
struct Username(String);

#[derive(serde::Deserialize)]
pub struct FormatData {
    email: Email,
    name: Username,
}

pub async fn subscribe(form: web::Form<FormatData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subsriber.",
        %request_id,
        subsriber_email = %form.email.0,
        subsriber_name = %form.name.0
    );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "[request_id {}] Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email.0,
        form.name.0
    );
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email.0,
        form.name.0,
        Utc::now()
    )
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "[request_id {}] New subscbier details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "[request_id {}] Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
