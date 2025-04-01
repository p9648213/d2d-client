use super::error::AppError;
use crate::utilities::db::{excute, query_optional};
use deadpool_postgres::Pool;
use tokio_postgres::Row;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub account_id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub image_url: Option<String>,
    pub provider: Option<String>,
}

impl User {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<String> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let account_id: Option<String> = row
            .try_get(format!("{}account_id", prefix).as_str())
            .unwrap_or(None);
        let username: Option<String> = row
            .try_get(format!("{}username", prefix).as_str())
            .unwrap_or(None);
        let email: Option<String> = row
            .try_get(format!("{}email", prefix).as_str())
            .unwrap_or(None);
        let password: Option<String> = row
            .try_get(format!("{}password", prefix).as_str())
            .unwrap_or(None);
        let image_url: Option<String> = row
            .try_get(format!("{}image_url", prefix).as_str())
            .unwrap_or(None);
        let provider: Option<String> = row
            .try_get(format!("{}provider", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            account_id,
            username,
            email,
            password,
            image_url,
            provider,
        }
    }

    pub async fn get_user_by_id(
        id: i32,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE id = $1", columns),
            &[&id],
            pool,
        )
        .await
    }

    pub async fn get_user_by_account_id(
        account_id: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE account_id = $1", columns),
            &[&account_id],
            pool,
        )
        .await
    }

    pub async fn get_user_by_email(
        email: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE email = $1", columns),
            &[&email],
            pool,
        )
        .await
    }

    pub async fn insert_user(
        id: &str,
        username: &str,
        password: &str,
        email: &str,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO users (id, username, password, email) VALUES ($1, $2, $3, $4)",
            &[&id, &username, &password, &email],
            pool,
        )
        .await
    }

    pub async fn insert_google_user(
        id: &str,
        account_id: &str,
        username: &str,
        email: &str,
        image_url: &str,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO users (id, account_id, username, email, image_url, provider) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &account_id, &username, &email, &image_url, &"google"],
            pool,
        )
        .await
    }

    pub async fn update_google_user_by_id(
        id: &str,
        account_id: &str,
        username: &str,
        image_url: &str,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE users SET account_id = $1, username = $2, image_url = $3, provider = $4 WHERE id = $5",
            &[&account_id, &username, &image_url, &"google", &id],
            pool,
        )
        .await
    }
}
