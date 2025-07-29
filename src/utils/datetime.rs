use chrono::{DateTime, FixedOffset, TimeZone};
use core::fmt;
pub trait TimeUtil {
    fn format_common_chinese(&self) -> String;
}
impl<Tz: TimeZone> TimeUtil for DateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn format_common_chinese(&self) -> String {
        let format = self
            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
            .format("%Y-%m-%d %H:%M:%S");
        format!("{format}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Utc};
    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn test1() {}
    #[test]
    fn sync_test1() {
        let time = Local::now();
        println!("{}", time.format_common_chinese());
        let time = Utc::now();
        println!("{}", time.format_common_chinese());
    }
}
