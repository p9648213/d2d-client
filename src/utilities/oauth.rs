use oauth2::{
	AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
	RedirectUrl, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
	StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
	basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
};
use reqwest::StatusCode;

use crate::{config::EnvConfig, models::error::AppError};

type GoogleClient = Client<
	StandardErrorResponse<BasicErrorResponseType>,
	StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
	StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
	StandardRevocableToken,
	StandardErrorResponse<RevocationErrorResponseType>,
	EndpointSet,
	EndpointNotSet,
	EndpointNotSet,
	EndpointNotSet,
	EndpointSet,
>;

pub fn create_google_client(config: &EnvConfig) -> Result<GoogleClient, AppError> {
	let base_url = &config.allow_origin;
	let client_id = ClientId::new(config.google_client_id.to_owned());
	let client_secret = ClientSecret::new(config.google_client_secret.to_owned());

	let auth_url =
		AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).map_err(|_| {
			AppError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Invalid authorization endpoint URL",
			)
		})?;

	let token_url =
		TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).map_err(|_| {
			AppError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Invalid token endpoint URL",
			)
		})?;

	let redirect_url = RedirectUrl::new(format!("{base_url}/auth/google/callback"))
		.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid redirect url"))?;

	let client = BasicClient::new(client_id)
		.set_client_secret(client_secret)
		.set_auth_uri(auth_url)
		.set_token_uri(token_url)
		.set_redirect_uri(redirect_url);

	Ok(client)
}
