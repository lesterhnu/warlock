use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{entity::{self, banner}, get_db, Result};

pub async fn get_banner_list() -> Result<Vec<entity::banner::Model>> {
    let db = get_db()?;
    let banners = entity::banner::Entity::find()
        .filter(banner::Column::IsShow.eq(1))
        .all(db)
        .await?;
    Ok(banners)
}