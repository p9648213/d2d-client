use axum::{
	Extension,
	response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;

use crate::{
	middlewares::auth_mw::UserAuth,
	views::pages::profile_v::{ProfilePageProps, render_profile_page},
};

pub async fn get_profile_page(
	Extension(user_auth): Extension<UserAuth>,
	token: CsrfToken,
) -> impl IntoResponse {
	let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

	let props = ProfilePageProps {
		authenticity_token,
		user_auth,
	};

	(token, Html(render_profile_page(&props).0))
}
