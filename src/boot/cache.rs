use lazy_static::lazy_static;
use redis::Client;
use crate::Result;

// 初始化redis client 并可以全局调用
lazy_static! {
    pub static ref REDIS_CLIENT: Client = new_redis();
}

pub fn new_redis()->Client{
    let client = Client::open(crate::CONFIG.get_string("redis.dsn").expect("redis dsn not found")).expect("redis connect error");
    client
}

pub fn get_redis_conn()->Result<redis::Connection>{
    let c = REDIS_CLIENT.get_connection()?;
    Ok(c)
}