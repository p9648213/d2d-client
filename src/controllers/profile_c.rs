use axum::{
    Extension,
    http::HeaderMap,
    response::Html,
};

use reqwest::StatusCode;

use crate::{
    middlewares::auth_mw::UserAuth,
    models::error::AppError,
    views::pages::profile_v::{ProfilePageProps, render_profile_page, render_profile_section},
};

pub async fn get_profile_page(
    Extension(user_auth): Extension<UserAuth>,
    headers: HeaderMap,
) -> Result<Html<String>, AppError> {
    if user_auth.0.is_none() {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"));
    }

    let boosted = headers.get("HX-Boosted");

    if boosted.is_some() {
        return Ok(Html(render_profile_section().0));
    }


    let props = ProfilePageProps {
        user_auth,
    };

    Ok(Html(render_profile_page(&props).0))
}
