use axum::{extract::Path, response::Html};

use crate::{
	models::error::AppError,
	views::pages::{home_v::render_home_section, profile_v::render_profile_section},
};

pub async fn get_section(Path(section): Path<String>) -> Result<Html<String>, AppError> {
	match section.as_str() {
		"home" => Ok(Html(render_home_section().0)),
		"profile" => Ok(Html(render_profile_section().0)),
		_ => Ok(Html("Not found".to_owned())),
	}
}
