use crate::{
    models::{
        auth::Claims,
        errors::{AppError, AuthError},
        StateType,
    },
    services::entry_service,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn get_entries(State(state): StateType, claims: Claims) -> Result<Json<Value>, AppError> {
    let id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;
    let entries = entry_service::get_entries(&state.db, id).await?;

    Ok(Json(json!({ "entries": entries })))
}
