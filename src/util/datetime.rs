use chrono::format::ParseErrorKind;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use core::fmt;

pub trait TimeUtil {
    fn to_common_chinese(&self) -> String;
}
impl<Tz: TimeZone> TimeUtil for DateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn to_common_chinese(&self) -> String {
        let format = self
            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
            .format("%Y-%m-%d %H:%M:%S");
        format!("{format}")
    }
}
fn from_string(time_str: &String) -> Result<DateTime<Utc>, ParseErrorKind> {
    if let Ok(datetime) = DateTime::parse_from_rfc3339(time_str) {
        return Ok(datetime.with_timezone(&Utc));
    }

    let formats = [
        "%Y-%m-%dT%H:%M:%S", // 格式 1: "2023-10-05T14:30:00"
        "%Y-%m-%d %H:%M:%S", // 格式 2: "2023-10-05 14:30:00"
        "%d/%m/%Y %H:%M",    // 格式 3: "05/10/2023 14:30"
        "%Y%m%dT%H%M%S",     // 格式 4: "20231005T143000"
        "%Y%m/%d/ %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%d %H:%M",
    ];

    for format in formats.iter() {
        if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(time_str, format) {
            if let Some(datetime_east_8) = Utc.from_local_datetime(&naive_datetime).single() {
                return Ok(datetime_east_8);
            }
        }
    }
    Err(ParseErrorKind::Impossible)
}

pub mod chinese_datetime_format {
    use crate::prelude::TimeUtil;
    use crate::util::datetime::from_string;
    use chrono::{DateTime, TimeZone, Utc};
    use core::fmt;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S, Tz: TimeZone>(date: &DateTime<Tz>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        Tz::Offset: fmt::Display,
    {
        let s = date.to_common_chinese();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_time = String::deserialize(deserializer)?;

        let date_time = from_string(&date_time)
            .map_err(|_| serde::de::Error::custom(format!("can not parse {date_time}")))?;
        Ok(date_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Utc};
    use serde::{Deserialize, Serialize};

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn test1() {}

    #[derive(Serialize, Deserialize, Debug)]
    struct MyStruct {
        #[serde(with = "chinese_datetime_format")]
        timestamp: DateTime<Utc>,
    }
    #[test]
    fn sync_test1() {
        let time = Local::now();
        println!("{}", time.to_common_chinese());
        let time = Utc::now();
        println!("{}", time.to_common_chinese());
    }
    #[test]
    fn main() {
        let data = MyStruct {
            timestamp: Utc.with_ymd_and_hms(2025, 9, 5, 14, 30, 0).unwrap(),
        };

        let json = serde_json::to_string(&data).unwrap();
        println!("{}", json); // {"timestamp":"2025-09-05 14:30:00"}
        let json = r#"{"timestamp":"2025-09-05 14:30:00"}"#;
        let parsed: MyStruct = serde_json::from_str(&json).unwrap();
        println!("{:?}", parsed);
        let json = r#"{"timestamp":"2025-09-05 14:30"}"#;
        let parsed: MyStruct = serde_json::from_str(&json).unwrap();
        println!("{:?}", parsed);
        let json = r#"{"timestamp":"2025-09-05T14:30"}"#;
        let parsed: MyStruct = serde_json::from_str(&json).unwrap();
        println!("{:?}", parsed);
    }
}
