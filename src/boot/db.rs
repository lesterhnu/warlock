use crate::{CONFIG, MyError, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub fn get_db<'a>() -> Result<&'a DatabaseConnection> {
    let res = DB.get().ok_or(MyError::from_msg("db error".to_string()))?;
    Ok(res)
}
pub async fn init_mysql()->crate::Result<()>{
    let dsn = format!(
        "mysql://{}:{}@{}:{}/{}",
        CONFIG.get_string("db.user")?,
        CONFIG.get_string("db.password")?,
        CONFIG.get_string("db.host")?,
        CONFIG.get_int("db.port")?,
        CONFIG.get_string("db.database")?
    );

    DB.get_or_init(|| async {
        let mut opt = ConnectOptions::new(dsn);
        opt.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);

        Database::connect(opt).await.expect("数据库打开失败")
    })
    .await;
    tracing::debug!("数据库连接成功");
    Ok(())
    // let db = sea_orm::Database::connect(dsn).await?;

}
