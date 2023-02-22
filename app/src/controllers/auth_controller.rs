use crate::{
    models::{
        auth::Credentials,
        errors::AppError,
        user::{NewUser, UserProfile},
        StateType,
    },
    services::auth_service,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn authorize(
    State(state): StateType,
    Json(credentials): Json<Credentials>,
) -> Result<Json<Value>, AppError> {
    let (token, user) = auth_service::authorize(&state.db, credentials).await?;

    Ok(Json(json!({
        "token": token,
        "profile": UserProfile::from(user)
    })))
}

pub async fn register(
    State(state): StateType,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
    let (token, new_user) = auth_service::register(&state.db, new_user).await?;

    Ok(Json(json!({
        "token": token,
        "profile": UserProfile::from(new_user)
    })))
}
