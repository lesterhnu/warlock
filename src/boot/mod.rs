pub mod cache;
pub mod conf;
pub mod db;
pub mod log;




use crate::Result;
use conf::load_config;
use tracing_appender::non_blocking::WorkerGuard;
pub async  fn init()->Result<WorkerGuard>{
    let cfg = load_config();
    let guard = log::register_log(cfg)?;
    db::init_mysql().await?;
    // cache::init_cache().await?;
    
    Ok(guard)
}