// use color_eyre::Result;
use time::{macros::format_description, UtcOffset};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
// use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::Result;

pub fn register_log(conf:config::Config) -> Result<WorkerGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(conf.get_string("log.level").unwrap_or("info".to_string())));
    let offset = UtcOffset::from_hms(8, 0, 0).unwrap_or(UtcOffset::UTC);
    let local_time = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second]"));
    let formatting_layer = fmt::layer()
        .with_file(true)
        .with_timer(local_time)
        .pretty()
        .with_writer(std::io::stdout);

    // file format
    let file_appender = rolling::daily(conf.get_string("log.dir")?, conf.get_string("log.log_file")?);
    
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    let file_time = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );
    let file_layer = fmt::layer()
        .with_timer(file_time)
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        //.with(ErrorLayer::default())
        .with(file_layer)
        .init();
    // color_eyre::install()?;
    Ok(guard)
}
