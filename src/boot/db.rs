use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use crate::{Result,CONFIG};
use tokio::sync::OnceCell;


pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn connect() -> Result<DatabaseConnection> {
    let dsn = format!("mysql://{}:{}@{}:{}/{}",
                      CONFIG.get_string("db.user")?,
                      CONFIG.get_string("db.password")?,
                      CONFIG.get_string("db.host")?,
                      CONFIG.get_int("db.port")?,
                      CONFIG.get_string("db.database")?);
    let db = sea_orm::Database::connect(dsn).await?;
    Ok(db)
}