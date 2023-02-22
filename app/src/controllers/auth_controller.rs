use crate::{
    entities::user::NewUser,
    models::{auth::Credentials, errors::AppError, StateType},
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
        "success": true,
        "token": token,
        "profile": {
            "id": user.id,
            "email": user.email,
            "firstname": user.firstname,
            "lastname": user.lastname,
        }
    })))
}

pub async fn register(
    State(state): StateType,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
    let (token, new_user) = auth_service::register(&state.db, new_user).await?;

    Ok(Json(json!({
        "success": true,
        "token": token,
        "profile": {
            "id": new_user.id,
            "email": new_user.email,
            "firstname": new_user.firstname,
            "lastname": new_user.lastname,
        }
    })))
}
