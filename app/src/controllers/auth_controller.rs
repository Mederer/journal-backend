use crate::{
    models::{
        auth::{Claims, Credentials},
        errors::{AppError, AuthError},
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

pub async fn validate_token(
    State(state): StateType,
    claims: Claims,
) -> Result<Json<Value>, AppError> {
    println!("Found this: {}", claims.sub);
    let id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;

    let user = auth_service::validate_token(&state.db, id).await?;

    Ok(Json(json!({ "user": user })))
}
