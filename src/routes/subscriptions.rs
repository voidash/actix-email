use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>,connection: web::Data<PgPool>) -> HttpResponse {
let request_id = Uuid::new_v4();
let request_span = tracing::info_span!(
    "Adding a new subscriber",
    %request_id,
    subscriber_email = %form.email,
    subscriber_name = %form.name
    );

let _request_span_guard = request_span.enter();
match sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#, 
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
).execute(connection.get_ref())
.await 
{
    Ok(_) => {
        log::info!("{} New subscriber details have been saved", request_id);
        HttpResponse::Ok().finish()
    },
    Err(_) => {
        log::error!("{} failed to execute query for user", request_id);
        HttpResponse::InternalServerError().finish()
    }
}
}

