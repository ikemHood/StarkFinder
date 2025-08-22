
mod libs {
    pub mod config;
    pub mod logging;
}
mod middlewares {
    pub mod request_id;
}
mod routes {
    pub mod health;
}

use axum::{
    http::{header::{CONTENT_TYPE, LOCATION}, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{TraceLayer, DefaultMakeSpan},
};

async fn root_redirect() -> impl IntoResponse {
    (StatusCode::MOVED_PERMANENTLY, [(LOCATION, "/health")])
}

#[tokio::main]
async fn main() {
    // Load .env first so RUST_LOG is respected
    let _ = dotenvy::dotenv();
    // JSON structured logs with RUST_LOG config
    libs::logging::init_tracing();

    let cfg = libs::config::AppConfig::from_env();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE]);

    // Router
    let app = Router::new()
        .route("/", get(root_redirect))
        .route("/health", get(routes::health::health));

    // request-id layers before trace
    let app = middlewares::request_id::add_request_id(app)
        // trace requests (include headers so x-request-id is visible)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)))

        .layer(cors);

    let addr = cfg.addr();
    let listener = TcpListener::bind(&addr).await.expect("bind failed");
    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server failed");
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
