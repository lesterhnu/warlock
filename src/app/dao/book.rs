use crate::{Result, get_db};
use crate::{dto, models::entity::book, pkg::utils};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryOrder, QuerySelect};
pub async fn get_books(page: u64, page_size: u64) -> Result<Vec<book::Model>> {
    let db = get_db()?;
    let books = book::Entity::find()
        .order_by_asc(book::Column::Id)
        .offset(page * page_size)
        .limit(page_size)
        .all(db)
        .await?;

    Ok(books)
}

pub async fn create_book(b: dto::book::Book) -> Result<book::Model> {
    let db = get_db()?;
    let am = book::ActiveModel {
        title: Set(b.title),
        author: Set(b.author),
        content: Set(b.content),
        chapter: Set(b.chapter),
        bg: Set(b.bg),
        create_time: Set(Some(utils::get_local_time())),
        ..Default::default()
    };
    let res = am.insert(db).await?;
    tracing::info!("create book success");
    Ok(res)
}
