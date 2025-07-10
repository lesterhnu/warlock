use crate::models::entity::book;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Book {
    pub id: i64,
    pub title: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub chapter: Option<String>,
    pub bg: Option<String>,
    pub create_time: Option<String>,
}

impl From<book::Model> for Book {
    fn from(m: book::Model) -> Self {
        Book {
            id: m.id,
            title: m.title,
            author: m.author,
            content: m.content,
            chapter: m.chapter,
            bg: m.bg,
            create_time: m.create_time.map(|t| t.to_string()),
        }
    }
}
