pub mod cache;
pub mod conf;
pub mod db;
pub mod log;
pub mod request;

use crate::Result;
use conf::load_config;
use lazy_static::lazy_static;
use tracing_appender::non_blocking::WorkerGuard;

lazy_static!{
    pub static ref ReqClient: reqwest::Client = request::new_request_client();

    
    // pub static ref Chrome:headless_chrome::Browser = headless_chrome::Browser::default().unwrap();
}
pub async  fn init()->Result<WorkerGuard>{
    let cfg = load_config();
    let guard = log::register_log(cfg)?;
    db::init_mysql().await?;
    // cache::init_cache().await?;
    
    Ok(guard)
}