use crate::{
    models::{
        auth::Claims,
        errors::{AppError, AuthError},
        StateType,
    },
    services::entry_service,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

pub async fn get_entries(State(state): StateType, claims: Claims) -> Result<Json<Value>, AppError> {
    let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;
    let entries = entry_service::get_entries(&state.db, user_id).await?;

    Ok(Json(json!({ "entries": entries })))
}

pub async fn delete_entry(
    State(state): StateType,
    claims: Claims,
    Path(entry_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;

    entry_service::delete_entry(&state.db, user_id, entry_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
