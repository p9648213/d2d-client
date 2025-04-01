use crate::{config::EnvConfig, models::error::AppError};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_csrf::CsrfToken;
use http_body_util::BodyExt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CsrfForm {
    authenticity_token: String,
}

pub async fn csrf_middleware(
    State(config): State<EnvConfig>,
    token: CsrfToken,
    method: Method,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    if method == Method::POST
        || method == Method::PUT
        || method == Method::DELETE
        || method == Method::PATCH
    {
        let csrf_header = request.headers().get("X-Csrf-Protection");

        if csrf_header.is_none() {
            tracing::error!("X-Csrf-Protection header is missing");
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        let origin = request.headers().get("Origin");

        if let Some(origin) = origin {
            let origin = origin.to_str().map_err(|error| {
                tracing::error!("Failed to get origin header: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

            if origin != config.allow_origin.as_str() {
                tracing::error!("Origin header is not allowed");
                return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
            }
        } else {
            tracing::error!("Origin header is missing");
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        let (parts, body) = request.into_parts();

        let bytes = body
            .collect()
            .await
            .map_err(|error| {
                tracing::error!("Failed to collect body: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?
            .to_bytes()
            .to_vec();

        let form: CsrfForm = serde_urlencoded::from_bytes(&bytes).map_err(|error| {
            tracing::error!("Failed to deserialize form: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        if token.verify(&form.authenticity_token).is_err() {
            tracing::error!("Authenticity token is invalid");
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        request = Request::from_parts(parts, Body::from(bytes));
    }

    Ok(next.run(request).await)
}
