use crate::{
    entities::user,
    models::{entry::NewEntry, errors::AppError, StateType},
    services::entry_service,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::{json, Value};

pub async fn get_entries(
    State(state): StateType,
    Extension(user): Extension<user::Model>,
) -> Result<Json<Value>, AppError> {
    let entries = entry_service::get_entries(&state.db, user).await?;

    Ok(Json(json!({ "entries": entries })))
}

pub async fn delete_entry(
    State(state): StateType,
    Extension(user): Extension<user::Model>,
    Path(entry_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    entry_service::delete_entry(&state.db, user, entry_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_entry(
    State(state): StateType,
    Extension(user): Extension<user::Model>,
    Json(new_entry): Json<NewEntry>,
) -> Result<Json<Value>, AppError> {
    let entry = entry_service::add_entry(&state.db, user, new_entry).await?;

    Ok(Json(json!({ "entry": entry })))
}
