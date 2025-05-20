use axum::{Extension, http::HeaderMap, response::Html};
use vy::IntoHtml;

use crate::{
    middlewares::auth_mw::UserAuth,
    views::pages::home_v::{HomePageProps, render_home_page, render_home_section},
};

pub async fn get_home_page(
    Extension(user_auth): Extension<UserAuth>,
    headers: HeaderMap,
) -> Html<String> {
    let boosted = headers.get("HX-Boosted");

    if boosted.is_some() {
        return Html(render_home_section().into_string());
    }

    let props = HomePageProps {
        user_info: user_auth.0,
    };

    Html(render_home_page(&props).into_string())
}
