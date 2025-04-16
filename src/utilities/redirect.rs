use axum::response::Response;
use reqwest::{StatusCode, header};

pub fn redirect_307(location: &str) -> Response {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, location)
        .body(axum::body::Body::empty())
        .unwrap()
}
