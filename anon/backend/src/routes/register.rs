use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::libs::{db::AppState, error::ApiError, wallet};

#[derive(Deserialize)]
pub struct RegisterReq {
    pub wallet: String,
    pub referral_code: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterRes {
    pub user_id: i64,
    pub wallet: String,
}

pub async fn register(
    State(AppState { pool }): State<AppState>,
    Json(req): Json<RegisterReq>,
) -> Result<impl IntoResponse, ApiError> {
    let normalized_wallet = wallet::normalize_and_validate(&req.wallet)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| ApiError::Internal("failed to start transaction"))?;

    // Insert user
    let rec = sqlx::query!(
        r#"INSERT INTO users (wallet) VALUES ($1)
           ON CONFLICT (wallet) DO NOTHING
           RETURNING id, wallet"#,
        normalized_wallet
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| crate::libs::error::map_sqlx_error(&e))?;

    let (user_id, wallet_saved) = match rec {
        Some(r) => (r.id, r.wallet),
        None => return Err(ApiError::Conflict("wallet already registered")),
    };

    // Insert profile
    sqlx::query!(
        r#"INSERT INTO profiles (user_id, referral_code) VALUES ($1, $2) ON CONFLICT (user_id) DO NOTHING"#,
        user_id,
        req.referral_code
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| crate::libs::error::map_sqlx_error(&e))?;

    tx.commit()
        .await
        .map_err(|_| ApiError::Internal("failed to commit transaction"))?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterRes {
            user_id,
            wallet: wallet_saved,
        }),
    ))
}
