use axum::{Extension, extract::Path, response::Html};
use reqwest::StatusCode;

use crate::{
	middlewares::auth_mw::UserAuth,
	models::error::AppError,
	views::pages::{home_v::render_home_section, profile_v::render_profile_section},
};

pub async fn get_section(
	Path(section): Path<String>,
	Extension(user_auth): Extension<UserAuth>,
) -> Result<Html<String>, AppError> {
	match section.as_str() {
		"home" => Ok(Html(render_home_section().0)),
		"profile" => {
			if user_auth.0.is_none() {
				return Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"));
			}
			Ok(Html(render_profile_section().0))
		}
		_ => Err(AppError::new(StatusCode::NOT_FOUND, "Not Found")),
	}
}
