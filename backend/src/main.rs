use axum::{http::HeaderValue, routing::get, Router};
use std::env;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;

#[derive(OpenApi)]
#[openapi(
        paths(handlers::get_health, handlers::get_random),
        components(schemas(handlers::Health, handlers::Random)),
        tags((name = "sveltekit-rust", description = "sveltekit-rust API")),
    )]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // init logging, start with `RUST_LOG=debug cargo watch -x run` to set log level
    #[cfg(debug_assertions)]
    dotenvy::from_path(".env.local").ok(); // we won't load .env in production
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/health", get(handlers::get_health))
        .route("/random", get(handlers::get_random))
        .layer(TraceLayer::new_for_http()) // logging/tracing
        .layer(
            // CORS
            env::var("ALLOWED_ORIGINS")
                .map(|s| {
                    CorsLayer::new().allow_origin(
                        s.split(',')
                            .map(|s| {
                                s.parse::<HeaderValue>().unwrap_or_else(|e| {
                                    panic!("Error: {}\n Invalid ALLOWED_ORIGINS origin: {}", s, e);
                                })
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap_or_else(|_| CorsLayer::new().allow_origin(AllowOrigin::any())), // if you don't specify ALLOWED_ORIGINS, allow any origin
        );

    axum::Server::bind(
        &env::var("HOST_ADDR_PORT")
            .unwrap_or_else(|e| {
                panic!("Error: {}\n HOST_ADDR_PORT not set", e);
            })
            .parse()
            .unwrap_or_else(|e| {
                panic!("Error: {}\n Invalid HOST_ADDR_PORT", e);
            }),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
