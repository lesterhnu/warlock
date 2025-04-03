use clap::Parser;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref CONFIG: config::Config = load_config();
}


#[derive(Parser)]
pub struct Args{
    #[clap(short, long, default_value = "config/config.toml")]
    pub conf_file:String
}


pub fn load_config()->config::Config{
    let args = Args::parse();
    let conf_file = args.conf_file;
    config::Config::builder()
        .add_source(config::File::with_name(&conf_file))
        .build()
        .expect("Failed to load config")
}