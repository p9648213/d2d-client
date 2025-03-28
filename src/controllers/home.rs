use axum::response::Html;

use crate::{models::error::AppError, views::home::render_home_page};

pub async fn home() -> Result<Html<String>, AppError> {
    Ok(Html(render_home_page()))
}
