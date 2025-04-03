use crate::{error::MyError, Result};

pub async fn ping() -> &'static str {
    "ping"
}
pub async fn test_config_error()->Result<()>{
    Err(MyError::ConfigError(config::ConfigError::NotFound("test".to_string())).into())
}