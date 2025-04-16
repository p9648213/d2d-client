use axum::{
    Extension,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;
use reqwest::StatusCode;

use crate::{
    middlewares::auth_mw::UserAuth,
    models::error::AppError,
    views::pages::profile_v::{ProfilePageProps, render_profile_page, render_profile_section},
};

pub async fn get_profile_page(
    Extension(user_auth): Extension<UserAuth>,
    token: CsrfToken,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    if user_auth.0.is_none() {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"));
    }

    let boosted = headers.get("HX-Boosted");

    if boosted.is_some() {
        return Ok(Html(render_profile_section().0).into_response());
    }

    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    let props = ProfilePageProps {
        authenticity_token,
        user_auth,
    };

    Ok((token, Html(render_profile_page(&props).0)).into_response())
}
