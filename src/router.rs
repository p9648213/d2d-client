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
    http::{HeaderValue, StatusCode, header},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_redispool::SessionRedisPool;
use deadpool_postgres::Pool;
use redis_pool::SingleRedisPool;
use tower_http::set_header::SetResponseHeaderLayer;

async fn ping() -> &'static str {
    "pong"
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub async fn create_router(
    pg_pool: Pool,
    redis_pool: SingleRedisPool,
    config: EnvConfig,
) -> Router {
    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let cfrs_key = config.csrf_encrypt_key.as_bytes();

    let session_key = config.session_encrypt_key.as_bytes();

    let database_key = config.database_encrypt_key.as_bytes();

    let cfrs_config = CsrfConfig::default().with_key(Some(
        axum_csrf::Key::try_from(cfrs_key).expect("Error while creating csrf key"),
    ));

    let session_config = SessionConfig::default()
        .with_key(
            axum_session::Key::try_from(session_key).expect("Error while creating session key"),
        )
        .with_database_key(
            axum_session::Key::try_from(database_key).expect("Error while creating session key"),
        );

    let session_store =
        SessionStore::<SessionRedisPool>::new(Some(redis_pool.clone().into()), session_config)
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
        .layer(CsrfLayer::new(cfrs_config))
        .layer(SessionLayer::new(session_store))
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .fallback(fallback)
}
