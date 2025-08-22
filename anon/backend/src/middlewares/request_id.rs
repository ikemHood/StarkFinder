use axum::Router;
use axum::http::header::HeaderName;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

pub fn add_request_id<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let x_request_id = HeaderName::from_static("x-request-id");
    router
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}
