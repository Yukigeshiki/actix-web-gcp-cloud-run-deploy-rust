use actix_web::HttpResponse;

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
name = "Performing health check",
)]
pub async fn health_check() -> HttpResponse {
    tracing::info!("Health check performed!");
    HttpResponse::Ok().finish()
}
