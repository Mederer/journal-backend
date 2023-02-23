use crate::{
    entities::entry,
    models::{entry::Entry, errors::AppError},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_entries(db: &DatabaseConnection, id: i32) -> Result<Vec<Entry>, AppError> {
    let entries = entry::Entity::find()
        .filter(entry::Column::UserId.eq(id))
        .all(db)
        .await?;

    Ok(entries)
}
