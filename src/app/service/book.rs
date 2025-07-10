use crate::dao;
use crate::models::dto::book::Book;
use crate::Result;

pub async fn get_books() -> Result<Vec<Book>> {
    dao::book::get_books().await
}