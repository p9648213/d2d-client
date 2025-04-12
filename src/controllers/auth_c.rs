use crate::{
	config::EnvConfig,
	contanst::{COOKIE_AUTH_CODE_VERIFIER, COOKIE_AUTH_CSRF_STATE},
	models::{error::AppError, user::User},
	utilities::{
		hash::{compare_password, hash_password},
		oauth::create_google_client,
	},
};
use axum::{
	Form,
	extract::{Query, State},
	http::StatusCode,
	response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
use cookie::Cookie;
use deadpool_postgres::Pool;
use oauth2::{AuthorizationCode, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
	pub email: String,
	pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
	pub username: String,
	pub email: String,
	pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
	code: String,
	state: String,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct GoogleUser {
	sub: String,
	name: String,
	email: Option<String>,
	email_verified: Option<bool>,
	picture: String,
}

pub async fn login(
	session: Session<SessionRedisPool>,
	State(pg_pool): State<Pool>,
	Form(login_form): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
	let row = User::get_user_by_email(
		&login_form.email,
		&pg_pool,
		vec!["id", "password", "username", "image_url"],
	)
	.await?;

	if let Some(row) = row {
		let user = User::try_from(&row, None);

		let user_password = user.password.ok_or_else(|| {
			tracing::error!("No password column or value is null");
			AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
		})?;

		let user_id = user.id.ok_or_else(|| {
			tracing::error!("No id column or value is null");
			AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
		})?;

		let user_name = user.username.unwrap_or("User".to_owned());

		let user_image = user
			.image_url
			.unwrap_or("/assets/images/default-user.webp".to_owned());

		if compare_password(&login_form.password, &user_password)? {
			session.set("user-id", user_id);
			session.set("user-image", user_image);
			session.set("user-name", user_name);

			let response = Response::builder()
				.status(StatusCode::OK)
				.header("HX-Location", "/")
				.body(axum::body::Body::empty())
				.unwrap();

			Ok(response)
		} else {
			Err(AppError::new(
				StatusCode::UNAUTHORIZED,
				"Invalid email or password",
			))
		}
	} else {
		Err(AppError::new(
			StatusCode::UNAUTHORIZED,
			"Invalid email or password",
		))
	}
}

pub async fn register(
	State(pg_pool): State<Pool>,
	Form(register_form): Form<RegisterForm>,
) -> impl IntoResponse {
	if register_form.email.is_empty()
		|| register_form.password.is_empty()
		|| register_form.username.is_empty()
	{
		return Err(AppError::new(
			StatusCode::BAD_REQUEST,
			"Input can not be empty",
		));
	}

	let row = User::get_user_by_email(&register_form.email, &pg_pool, vec!["id"]).await?;

	if row.is_some() {
		return Err(AppError::new(
			StatusCode::UNAUTHORIZED,
			"Email already exists",
		));
	}

	let password_hash = hash_password(&register_form.password)?;

	User::insert_user(
		&Uuid::new_v4().to_string(),
		&register_form.username,
		&register_form.email,
		&password_hash,
		&pg_pool,
	)
	.await?;

	Ok([(
		"HX-Trigger",
		r#"{"toastmessage":{"type":"success","message":"User create successfully"}}"#,
	)])
}

pub async fn logout(session: Session<SessionRedisPool>) -> Result<impl IntoResponse, AppError> {
	session.destroy();

	let response = Response::builder()
		.status(StatusCode::OK)
		.header("HX-Location", "/")
		.body(axum::body::Body::empty())
		.unwrap();

	Ok(response)
}

pub async fn google_login(State(config): State<EnvConfig>) -> Result<impl IntoResponse, AppError> {
	let client = create_google_client(&config)?;

	let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

	let (authorize_url, csrf_state) = client
		.authorize_url(oauth2::CsrfToken::new_random)
		.add_scope(Scope::new(
			"https://www.googleapis.com/auth/userinfo.profile".to_string(),
		))
		.add_scope(Scope::new(
			"https://www.googleapis.com/auth/userinfo.email".to_string(),
		))
		.set_pkce_challenge(pkce_code_challenge)
		.url();

	let cookie_max_age = cookie::time::Duration::minutes(5);
	let crsf_cookie: Cookie = Cookie::build((COOKIE_AUTH_CSRF_STATE, csrf_state.secret().to_owned()))
		.http_only(true)
		.path("/")
		.same_site(cookie::SameSite::Lax)
		.max_age(cookie_max_age)
		.into();

	let code_verifier: Cookie = Cookie::build((
		COOKIE_AUTH_CODE_VERIFIER,
		pkce_code_verifier.secret().to_owned(),
	))
	.http_only(true)
	.path("/")
	.same_site(cookie::SameSite::Lax)
	.max_age(cookie_max_age)
	.into();

	let cookies = CookieJar::new().add(crsf_cookie).add(code_verifier);

	Ok((cookies, Redirect::to(authorize_url.as_str())))
}

pub async fn google_callback(
	session: Session<SessionRedisPool>,
	State(pg_pool): State<Pool>,
	State(config): State<EnvConfig>,
	Query(query): Query<AuthRequest>,
	cookies: CookieJar,
) -> Result<impl IntoResponse, AppError> {
	let client_url = &config.allow_origin;
	let code = query.code;
	let state = query.state;
	let stored_state = cookies.get(COOKIE_AUTH_CSRF_STATE);
	let stored_code_verifier = cookies.get(COOKIE_AUTH_CODE_VERIFIER);

	let (Some(csrf_state), Some(code_verifier)) = (stored_state, stored_code_verifier) else {
		return Ok(StatusCode::BAD_REQUEST.into_response());
	};

	if csrf_state.value() != state {
		return Ok(StatusCode::BAD_REQUEST.into_response());
	}

	let client = create_google_client(&config)?;
	let code = AuthorizationCode::new(code);
	let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_owned());

	let http_client = oauth2::reqwest::ClientBuilder::new()
		.redirect(oauth2::reqwest::redirect::Policy::none())
		.build()
		.expect("Client should build");

	let token_response = client
		.exchange_code(code)
		.set_pkce_verifier(pkce_code_verifier)
		.request_async(&http_client)
		.await
		.map_err(|_| {
			AppError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to get token response",
			)
		})?;

	let google_user = oauth2::reqwest::Client::new()
		.get("https://www.googleapis.com/oauth2/v3/userinfo")
		.bearer_auth(token_response.access_token().secret())
		.send()
		.await
		.map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user info"))?
		.json::<GoogleUser>()
		.await
		.map_err(|_| {
			AppError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to convert user info to Json",
			)
		})?;

	let account_id = google_user.sub;

	let row =
		User::get_user_by_account_id(&account_id, &pg_pool, vec!["id", "username", "image_url"])
			.await?;

	match row {
		Some(row) => {
			let user = User::try_from(&row, None);

			let user_id = user.id.ok_or_else(|| {
				tracing::error!("No id column or value is null");
				AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
			})?;

			let user_name = user.username.ok_or_else(|| {
				tracing::error!("No username column or value is null");
				AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
			})?;

			let user_image = user.image_url.ok_or_else(|| {
				tracing::error!("No image_url column or value is null");
				AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
			})?;

			session.set("user-id", user_id);
			session.set("user-name", user_name);
			session.set("user-image", user_image);
		}
		None => {
			if let Some(email) = google_user.email {
				let row = User::get_user_by_email(&email, &pg_pool, vec!["id"]).await?;

				match row {
					Some(row) => {
						let user = User::try_from(&row, None);

						let user_id = user.id.ok_or_else(|| {
							tracing::error!("No id column or value is null");
							AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
						})?;

						let result = User::update_google_user_by_id(
							&user_id,
							&account_id,
							&google_user.name,
							&google_user.picture,
							&pg_pool,
						)
						.await?;

						if result == 0 {
							return Err(AppError::new(
								StatusCode::INTERNAL_SERVER_ERROR,
								"Failed to create google users",
							));
						}

						session.set("user-id", user_id);
						session.set("user-name", &google_user.name);
						session.set("user-image", &google_user.picture);
					}
					None => {
						let uuid = Uuid::new_v4().to_string();

						let result = User::insert_google_user(
							&uuid,
							&account_id,
							&google_user.name,
							&email,
							&google_user.picture,
							&pg_pool,
						)
						.await?;

						if result == 0 {
							return Err(AppError::new(
								StatusCode::INTERNAL_SERVER_ERROR,
								"Failed to create google users",
							));
						}

						session.set("user-id", uuid);
						session.set("user-name", &google_user.name);
						session.set("user-image", &google_user.picture);
					}
				}
			} else {
				let uuid = Uuid::new_v4().to_string();

				let result = User::insert_google_user(
					&uuid,
					&account_id,
					&google_user.name,
					&google_user.email.unwrap_or_default(),
					&google_user.picture,
					&pg_pool,
				)
				.await?;

				if result == 0 {
					return Err(AppError::new(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to create google users",
					));
				}

				session.set("user-id", uuid);
				session.set("user-name", &google_user.name);
				session.set("user-image", &google_user.picture);
			}
		}
	};

	let mut remove_csrf_cookie = Cookie::new(COOKIE_AUTH_CSRF_STATE, "");
	remove_csrf_cookie.set_path("/");
	remove_csrf_cookie.make_removal();

	let mut remove_code_verifier = Cookie::new(COOKIE_AUTH_CODE_VERIFIER, "");
	remove_code_verifier.set_path("/");
	remove_code_verifier.make_removal();

	let cookies = CookieJar::new()
		.add(remove_csrf_cookie)
		.add(remove_code_verifier);

	let response = (cookies, Redirect::to(client_url.as_str())).into_response();

	Ok(response)
}
