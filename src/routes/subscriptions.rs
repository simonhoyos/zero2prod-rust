use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

// Serialize the body from a request
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Create a new subscription based based on the name and email sent to the request.
// Content-Type are defined in web. For example: web::Json<FormData> deserializes a JSON body.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(v) => {
            println!("Value created: {:?}", v);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}
