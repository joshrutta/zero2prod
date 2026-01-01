use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError, web};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Context;
use crate::routes::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String
}

#[derive(thiserror::Error)]
pub enum SubscribeConfirmError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is no subscriber associated with the provided token.")]
    UnknownToken,
}

impl std::fmt::Debug for SubscribeConfirmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeConfirmError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscribeConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeConfirmError::UnknownToken => StatusCode::UNAUTHORIZED,
        }
    }
}

#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(parameters, pool)
)]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SubscribeConfirmError> {
    let subscriber_id = get_subscriber_id_from_token(
        &pool,
        &parameters.subscription_token
    )
        .await
        .context("Failed to retreive subscriber id from token")?
        .ok_or(SubscribeConfirmError::UnknownToken)?;
    confirm_subscriber(&pool, subscriber_id)
        .await
        .context("Failed to confirm subscriber")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Mark subscriber as confirmed",
    skip(subscriber_id, pool)
)]
pub async fn confirm_subscriber(
    pool: &PgPool,
    subscriber_id: Uuid
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
        )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
        WHERE subscription_token = $1",
        subscription_token
    )
        .fetch_optional(pool)
        .await?;
    Ok(result.map(|r| r.subscriber_id))
}