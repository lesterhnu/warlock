use clap::Parser;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: config::Config = load_config();
    pub static ref CFG: Cfg = CONFIG.clone().try_deserialize::<Cfg>().unwrap();
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, default_value = "config/config.toml")]
    pub conf_file: String,
}

pub fn load_config() -> config::Config {
    let args = Args::parse();
    let conf_file = args.conf_file;
    config::Config::builder()
        .add_source(config::File::with_name(&conf_file))
        .build()
        .expect("Failed to load config")
}

#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub app: App,
    pub log: Log,
    pub db: Db,
    pub redis: Redis,
}
#[derive(Debug, Deserialize)]
pub struct App {
    pub app_name: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
    pub dir: String,
    pub log_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Db{
    #[serde(rename = "type")]
    pub db_type:String,
    pub host:String,
    pub port:i32,
    pub user:String,
    pub password:String,
    pub database:String,
}

#[derive(Debug, Deserialize)]
pub struct Redis{
    pub host:String,
    pub port:i32,
}