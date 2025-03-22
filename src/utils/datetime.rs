use chrono::{DateTime, TimeZone, Utc};
use core::fmt;
use std::time::SystemTime;

pub fn now() -> DateTime<Utc> {
    SystemTime::now().into()
}

pub fn format_common_chinese<Tz: TimeZone>(date_time: &DateTime<Tz>) -> String
where
    Tz::Offset: fmt::Display,
{
    let format = date_time.format("%Y-%m-%d %H:%M:%S");
    format!("{format}")
}
