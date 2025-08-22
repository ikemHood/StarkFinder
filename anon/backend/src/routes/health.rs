use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub ok: bool,
    pub versions: &'static str,
}

pub async fn health() -> impl IntoResponse {
    Json(Health {
        ok: true,
        versions: env!("CARGO_PKG_VERSION"),
    })
}
