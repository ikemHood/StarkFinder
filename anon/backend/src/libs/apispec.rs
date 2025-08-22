use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::health,
        crate::routes::register::register
    ),
    components(
        schemas(
            crate::routes::health::Health,
            crate::routes::register::RegisterReq,
            crate::routes::register::RegisterRes,
            crate::libs::error::ErrorBody
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication & registration endpoints")
    )
)]
pub struct ApiDoc;
