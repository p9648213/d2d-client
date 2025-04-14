use axum::{
	Extension,
	response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;
use reqwest::StatusCode;

use crate::{
	middlewares::auth_mw::UserAuth,
	models::error::AppError,
	views::pages::profile_v::{ProfilePageProps, render_profile_page},
};

pub async fn get_profile_page(
	Extension(user_auth): Extension<UserAuth>,
	token: CsrfToken,
) -> Result<impl IntoResponse, AppError> {
	if user_auth.0.is_none() {
		return Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"));
	}

	let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

	let props = ProfilePageProps {
		authenticity_token,
		user_auth,
	};

	Ok((token, Html(render_profile_page(&props).0)))
}
