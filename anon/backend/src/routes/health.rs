use axum::{response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Health {
    pub ok: bool,
    pub versions: &'static str,
}

/// Health check
#[utoipa::path(
    get,
    tag = "health",
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = Health)
    )
)]
pub async fn health() -> impl IntoResponse {
    Json(Health {
        ok: true,
        versions: env!("CARGO_PKG_VERSION"),
    })
}
