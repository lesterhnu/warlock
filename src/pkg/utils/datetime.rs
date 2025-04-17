use sea_orm::prelude::TimeDateTime;
use time::format_description;
use time::format_description::FormatItem;
use time::macros::offset;

// 获取时间format
pub fn get_time_format() -> Vec<FormatItem<'static>> {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    format
}

// 获取当前时间datetime, +8时区；
pub fn get_local_time() -> TimeDateTime {
    let now = time::OffsetDateTime::now_utc().to_offset(offset!(+8));
    time::PrimitiveDateTime::new(now.date(), now.time())
}