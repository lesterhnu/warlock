use crate::models::dto::book::Book;
use crate::models::entity::book;
use sea_orm::{ EntityTrait, PaginatorTrait, QueryOrder };
use crate::{Result,get_db};
pub async fn get_books()->Result<Vec<Book>>{
    let db = get_db()?;
    let mut book_pages = book::Entity::find()
        .order_by_asc(book::Column::Id)
        .paginate(db,10);
    let mut res = Vec::new();
    while let Some(books) = book_pages.fetch_and_next().await? {
        for b in books{
            res.push(Book::from(b));
        }
    }
    
    Ok(res)
}