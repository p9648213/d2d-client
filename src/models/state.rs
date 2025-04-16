use axum::extract::FromRef;
use deadpool_postgres::Pool;

use crate::config::EnvConfig;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pg_pool: Pool,
    pub config: EnvConfig,
}
