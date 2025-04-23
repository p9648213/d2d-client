use std::time::Duration;

use crate::{
    config::EnvConfig,
    controllers::{
        auth_c::{google_callback, google_login, login, logout, register},
        home_c::get_home_page,
        profile_c::get_profile_page,
    },
    middlewares::{auth_mw::auth_middleware, csrf_mw::csrf_middleware},
    models::state::AppState,
};
use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Request, Response, StatusCode, header},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_redis_bb8_pool::{SessionRedisPool, SessionRedisSessionStore};
use deadpool_postgres::Pool;
use tower_http::{
    classify::ServerErrorsFailureClass, set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tracing::Span;

async fn ping() -> &'static str {
    "pong"
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub async fn create_router(
    pg_pool: Pool,
    redis_pool: SessionRedisPool,
    config: EnvConfig,
) -> Router {
    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let session_key = config.session_encrypt_key.as_bytes();

    let database_key = config.database_encrypt_key.as_bytes();

    let session_config = SessionConfig::default()
        .with_key(
            axum_session::Key::try_from(session_key).expect("Error while creating session key"),
        )
        .with_database_key(
            axum_session::Key::try_from(database_key).expect("Error while creating session key"),
        );

    let session_store =
        SessionRedisSessionStore::new(Some(redis_pool), session_config)
            .await
            .expect("Error while creating session store");

    let app_state = AppState { pg_pool, config };

    let auth_route = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/logout", post(logout))
        .route("/auth/google/login", get(google_login))
        .route("/auth/google/callback", get(google_callback));

    Router::new()
        .route("/", get(get_home_page))
        .route("/profile", get(get_profile_page))
        .merge(auth_route)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(SessionLayer::new(session_store))
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        )
}

fn on_request(request: &Request<Body>, _: &Span) {
    tracing::info!(
        "-> Request : method {} path {}",
        request.method(),
        request.uri().path()
    )
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!("<- Response: status {} in {:?}", response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency)
}
