use crate::{
    config::EnvConfig, controllers::home::home, middlewares::csrf::csrf_middleware,
    models::state::AppState,
};
use axum::{
    Router,
    http::{HeaderValue, StatusCode, header},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::get,
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use deadpool_postgres::Pool;
use tower_http::set_header::SetResponseHeaderLayer;

async fn ping() -> &'static str {
    "pong"
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub async fn create_router(pg_pool: Pool, config: EnvConfig) -> Router {
    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let cfrs_key = config.csrf_encrypt_key.as_bytes();

    let cfrs_config = CsrfConfig::default().with_key(Some(
        axum_csrf::Key::try_from(cfrs_key).expect("Error while creating csrf key"),
    ));

    let app_state = AppState { pg_pool, config };

    Router::new()
        .route("/", get(home))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_config))
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .fallback(fallback)
}
