use axum::response::Html;
use axum_csrf::CsrfToken;

use crate::views::home_v::render_home_page;

pub async fn get_home_page(token: CsrfToken) -> Html<String> {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());
    Html(render_home_page(authenticity_token).0)
}
