use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "log.txt".to_string());

    let result = find_inserted_after_confirmed(&path);
    println!("== inserted-after-confirmed ==");
    println!("count: {}", result.len());
    for tx in &result {
        println!("{}", tx);
    }

    let durations = time_first_insert_to_confirm(&path);
    println!();
    println!("== first-insert -> confirm (ms) ==");
    for (sig, ms) in &durations {
        println!("{}\t{} ms", sig, ms);
    }
    if !durations.is_empty() {
        let mut ds: Vec<i64> = durations.iter().map(|(_, d)| *d).collect();
        ds.sort();
        let n = ds.len();
        let sum: i64 = ds.iter().sum();
        let avg = sum as f64 / n as f64;
        let median = if n % 2 == 1 {
            ds[n / 2] as f64
        } else {
            (ds[n / 2 - 1] + ds[n / 2]) as f64 / 2.0
        };
        println!(
            "-- n={} min={} max={} avg={:.1} median={:.1} (ms)",
            n,
            ds[0],
            ds[n - 1],
            avg,
            median
        );
    }
    let vec = find_inserted_after_confirmed(path);
    println!("wrong inserted: {:?}", vec);
}

/// 读取日志文件,找出在 `confirmed` 之后仍然出现 `inserted` 的 tx 签名。
fn find_inserted_after_confirmed<P: AsRef<Path>>(path: P) -> Vec<String> {
    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read {:?}: {}", path.as_ref(), e);
            return Vec::new();
        }
    };

    let mut confirmed: HashSet<String> = HashSet::new();
    let mut offenders: Vec<String> = Vec::new();
    let mut seen_offender: HashSet<String> = HashSet::new();

    for line in content.lines() {
        // 找到 "<pending>" 之后的有效负载
        let payload = match line.find("<pending>") {
            Some(idx) => line[idx + "<pending>".len()..].trim(),
            None => continue,
        };

        if let Some(rest) = payload.strip_prefix("tx:") {
            // confirmed 行: <pending> tx:<SIG> confirmed
            if let Some(sig) = rest.split_whitespace().next()
                && rest.contains("confirmed")
            {
                confirmed.insert(sig.to_string());
            }
        } else {
            // inserted 行: <pending> <SIG> inserted ,token:...,wallet:...
            let mut it = payload.split_whitespace();
            let sig = match it.next() {
                Some(s) => s,
                None => continue,
            };
            let kind = it.next().unwrap_or("");
            if kind == "inserted"
                && confirmed.contains(sig)
                && seen_offender.insert(sig.to_string())
            {
                offenders.push(sig.to_string());
            }
        }
    }

    offenders
}

/// 统计每个 tx 从"第一次 inserted"到"confirmed"的耗时 (毫秒)。
/// 只统计在日志中同时出现了 inserted 与 confirmed 的 tx,且以首次 inserted 为起点、
/// 首次 confirmed 为终点。
fn time_first_insert_to_confirm<P: AsRef<Path>>(path: P) -> Vec<(String, i64)> {
    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read {:?}: {}", path.as_ref(), e);
            return Vec::new();
        }
    };

    let mut first_insert: HashMap<String, i64> = HashMap::new();
    let mut seen_confirm: HashSet<String> = HashSet::new();
    let mut results: Vec<(String, i64)> = Vec::new();

    for line in content.lines() {
        let ts = match parse_ts_millis(line) {
            Some(t) => t,
            None => continue,
        };
        let payload = match line.find("<pending>") {
            Some(idx) => line[idx + "<pending>".len()..].trim(),
            None => continue,
        };

        if let Some(rest) = payload.strip_prefix("tx:") {
            if rest.contains("confirmed")
                && let Some(sig) = rest.split_whitespace().next()
                && seen_confirm.insert(sig.to_string())
                && let Some(&start) = first_insert.get(sig)
            {
                results.push((sig.to_string(), ts - start));
            }
        } else {
            let mut it = payload.split_whitespace();
            let sig = match it.next() {
                Some(s) => s,
                None => continue,
            };
            if it.next() == Some("inserted") {
                first_insert.entry(sig.to_string()).or_insert(ts);
            }
        }
    }

    results
}

/// 解析行首的时间戳 `YYYY-MM-DD HH:MM:SS.mmm` 为毫秒数。
fn parse_ts_millis(line: &str) -> Option<i64> {
    if line.len() < 23 {
        return None;
    }
    let b = line.as_bytes();
    if b[4] != b'-'
        || b[7] != b'-'
        || b[10] != b' '
        || b[13] != b':'
        || b[16] != b':'
        || b[19] != b'.'
    {
        return None;
    }
    let year: i64 = line[0..4].parse().ok()?;
    let month: i64 = line[5..7].parse().ok()?;
    let day: i64 = line[8..10].parse().ok()?;
    let hour: i64 = line[11..13].parse().ok()?;
    let min: i64 = line[14..16].parse().ok()?;
    let sec: i64 = line[17..19].parse().ok()?;
    let ms: i64 = line[20..23].parse().ok()?;
    let days = days_from_civil(year, month, day);
    Some(days * 86_400_000 + hour * 3_600_000 + min * 60_000 + sec * 1_000 + ms)
}

/// Howard Hinnant 公历日期 -> 距 1970-01-01 的天数。
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
