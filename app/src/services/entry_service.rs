use crate::{
    entities::{entry, user},
    models::{entry::NewEntry, errors::AppError},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, ModelTrait, QueryFilter, Set};

pub async fn get_entries(
    db: &DatabaseConnection,
    user: user::Model,
) -> Result<Vec<entry::Model>, AppError> {
    let entries = user.find_related(entry::Entity).all(db).await?;

    Ok(entries)
}

pub async fn delete_entry(
    db: &DatabaseConnection,
    user: user::Model,
    entry_id: i32,
) -> Result<(), AppError> {
    let entry = user
        .find_related(entry::Entity)
        .filter(entry::Column::Id.eq(entry_id))
        .one(db)
        .await?;

    if let Some(entry) = entry {
        entry.delete(db).await?;
        Ok(())
    } else {
        Err(AppError::EntityNotFound)
    }
}

pub async fn add_entry(
    db: &DatabaseConnection,
    user: user::Model,
    new_entry: NewEntry,
) -> Result<entry::Model, AppError> {
    let entry = entry::ActiveModel {
        title: Set(new_entry.title),
        body: Set(new_entry.body),
        user_id: Set(user.id),
        ..Default::default()
    };

    let entry = entry.insert(db).await?;

    Ok(entry)
}
