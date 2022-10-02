use actix_web::HttpResponse;

// Respond 200 if the server is up and running.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
