use axum::{
	Extension,
	http::HeaderMap,
	response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;

use crate::{
	middlewares::auth_mw::UserAuth,
	views::pages::home_v::{HomePageProps, render_home_page, render_home_section},
};

pub async fn get_home_page(
	Extension(user_auth): Extension<UserAuth>,
	token: CsrfToken,
	headers: HeaderMap,
) -> impl IntoResponse {
	let boosted = headers.get("HX-Boosted");

	if boosted.is_some() {
		return Html(render_home_section().0).into_response();
	}

	let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

	let props = HomePageProps {
		authenticity_token,
		user_auth,
	};

	(token, Html(render_home_page(&props).0)).into_response()
}
