use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TransactionTrait};

use crate::{entity::{self, banner}, get_db, Result};

pub async fn get_banner_list() -> Result<Vec<entity::banner::Model>> {
    let db = get_db()?;
    let banners = entity::banner::Entity::find()
        .filter(banner::Column::IsShow.eq(1))
        .all(db)
        .await?;
    Ok(banners)
}

pub async fn create_banner(list:Vec<entity::banner::Model>)->Result<i32>{
    let db = get_db()?;
    let txn = db.begin().await?;
    let active_model_list:Vec<entity::banner::ActiveModel> = list.into_iter().map(|item|item.into_active_model()).collect();
    let res = entity::banner::Entity::insert_many(active_model_list).exec(&txn).await?;

    txn.commit().await?;
    Ok(res.last_insert_id as i32)
}

pub async fn delete_banner(ids:Vec<i32>)->Result<()>{
    let db = get_db()?;
    entity::banner::Entity::delete_many()
        .filter(banner::Column::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(())
}