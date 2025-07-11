use crate::{dao, dto};
use crate::models::dto::book::Book;
use crate::Result;

pub async fn get_books(p: dto::CommonReq) -> Result<Vec<Book>> {
    let books = dao::book::get_books(p.page.unwrap_or(0),p.page_size.unwrap_or(10)).await?;
    let mut res = Vec::new();
    for b in books.into_iter() {
        res.push(Book::from(b))
    }
    tracing::info!("get books success");
    Ok(res)
}

pub async fn create_book(b: dto::book::Book) -> Result<()> {
    dao::book::create_book(b).await?;
    tracing::info!("create book success");
    Ok(())
}