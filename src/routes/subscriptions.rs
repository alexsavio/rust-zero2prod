use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    tracing::info!("Adding '{}' '{}' as a new subscriber.", form.email, form.name);
    tracing::info!("Saving new subscriber details to the database.");
    let subscriber_id = Uuid::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        subscriber_id,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("Saved new subscriber details to the database.");
            HttpResponse::Ok()
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}.", e);
            HttpResponse::InternalServerError()
        }
    }
}
