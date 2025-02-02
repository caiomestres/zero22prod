use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Form é um tipo de extrator
// Actix-web pode fazer até 10 tipos diferentes de extratores por handle
pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        Values($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await{
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute the query: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    };
    //HttpResponse:Ok().finish() pode omitir o .finish
    // println!("{}, {}", form.email, form.name);
    HttpResponse::Ok().finish()
}
