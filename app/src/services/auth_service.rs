use crate::{
    entities::user,
    models::user::{NewUser, User},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    helpers::create_token,
    models::{
        auth::Credentials,
        errors::{AppError, AuthError},
    },
};

pub async fn authorize(
    db: &DatabaseConnection,
    credentials: Credentials,
) -> Result<(String, User), AppError> {
    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(credentials.email))
                .add(user::Column::Secret.eq(credentials.secret)),
        )
        .one(db)
        .await?;

    if let Some(user) = user {
        Ok((create_token(user.id.to_string().as_str())?, user))
    } else {
        Err(AuthError::InvalidCredentials.into())
    }
}

pub async fn register(
    db: &DatabaseConnection,
    new_user: NewUser,
) -> Result<(String, User), AppError> {
    let new_user = user::ActiveModel {
        firstname: Set(new_user.firstname),
        lastname: Set(new_user.lastname),
        email: Set(new_user.email),
        secret: Set(new_user.secret),
        ..Default::default()
    };

    let new_user = new_user.insert(db).await?;
    let token = create_token(new_user.id.to_string().as_str())?;

    Ok((token, new_user))
}

pub async fn validate_token(db: &DatabaseConnection, id: i32) -> Result<User, AppError> {
    let user = user::Entity::find_by_id(id).one(db).await?;

    if let Some(user) = user {
        Ok(user)
    } else {
        Err(AuthError::InvalidToken.into())
    }
}
