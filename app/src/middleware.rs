use axum::{extract::State, http::Request, middleware::Next, response::Response};

use crate::{
    models::{
        auth::Claims,
        errors::{AppError, AuthError},
        StateType,
    },
    services::auth_service,
};

pub async fn extract_user<B>(
    State(state): StateType,
    claims: Claims,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;
    let user = auth_service::validate_token(&state.db, user_id).await?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
