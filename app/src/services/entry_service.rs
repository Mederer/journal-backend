use crate::{entities::entry, models::errors::AppError};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

pub async fn get_entries(db: &DatabaseConnection, id: i32) -> Result<Vec<entry::Model>, AppError> {
    let entries = entry::Entity::find()
        .filter(entry::Column::UserId.eq(id))
        .all(db)
        .await?;

    Ok(entries)
}

pub async fn delete_entry(
    db: &DatabaseConnection,
    user_id: i32,
    entry_id: i32,
) -> Result<(), AppError> {
    let entry = entry::Entity::find_by_id(entry_id)
        .filter(entry::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    if let Some(entry) = entry {
        entry.delete(db).await?;
        Ok(())
    } else {
        Err(AppError::EntityNotFound)
    }
}
