use std::collections::BTreeMap;
use std::fs;

//     watcher_20260531_1.log 这个文件 统计 日志文件 内 所有token 的 [Pre],[Pnl],[End] 平均耗时
//     end eg: 2026-06-08 16:51:25.595 INFO  1 --- [redis-queue-80] com.orca.task.evm.StatisticTask:156 :      [End]   [8453]  [46378002]      [0xbd285498822217835d839660583c794fc5419c19]    [10301ms]
//     pre eg: 2026-06-08 16:51:25.737 INFO  1 --- [redis-queue-40] com.orca.task.evm.PretreatmentTask:95 :    [Pre]   [8453]  [46378016]      [0xbd285498822217835d839660583c794fc5419c19]    [12465ms]
//     pnl eg: 2026-06-08 16:51:23.478 INFO  1 --- [redis-queue-37] com.orca.task.evm.StatisticTask:818 :      [Pnl]   [8453]  [46538079]      [0x0326b14b5744e160b9269c6a4a028c30c851d362]    [1-0]   [1-0]   [0-0]
//     注意: Pnl 行不带 [Nms] 耗时字段,故只统计出现次数,平均耗时不可用。

const KINDS: [&str; 3] = ["Pre", "Pnl", "End"];

#[derive(Default, Clone, Copy)]
struct Stat {
    count: u64,
    total_ms: u64,
}

/// 提取一行中所有 `[...]` 方括号内的内容
fn brackets(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        rest = &rest[open + 1..];
        if let Some(close) = rest.find(']') {
            out.push(&rest[..close]);
            rest = &rest[close + 1..];
        } else {
            break;
        }
    }
    out
}

fn main() {
    let path = "src/bin/watcher_20260608_3.log";
    let content = fs::read_to_string(path).unwrap();

    // token -> [Pre, Pnl, End] 的统计
    let mut by_token: BTreeMap<String, [Stat; 3]> = BTreeMap::new();
    let mut totals = [Stat::default(); 3];

    for line in content.lines() {
        let fields = brackets(line);

        let Some(kind_idx) = KINDS.iter().position(|k| fields.iter().any(|f| f == k)) else {
            continue;
        };

        let Some(token) = fields.iter().find(|f| f.starts_with("0x")) else {
            continue;
        };

        let stat = &mut by_token.entry(token.to_string()).or_default()[kind_idx];
        stat.count += 1;
        totals[kind_idx].count += 1;

        // 仅 Pre/End 带 [Nms] 耗时字段
        if let Some(ms) = fields
            .iter()
            .find_map(|f| f.strip_suffix("ms").and_then(|n| n.parse::<u64>().ok()))
        {
            stat.total_ms += ms;
            totals[kind_idx].total_ms += ms;
        }
    }

    let fmt = |s: &Stat| -> String {
        if s.total_ms > 0 {
            format!("count={:<6} avg={}ms", s.count, s.total_ms / s.count)
        } else {
            format!("count={:<6} avg=n/a", s.count)
        }
    };

    println!("==== 按 token 分组 ====");
    for (token, stats) in &by_token {
        println!("{token}");
        for (i, kind) in KINDS.iter().enumerate() {
            if stats[i].count > 0 {
                println!("    [{kind}] {}", fmt(&stats[i]));
            }
        }
    }

    println!("\n==== 全局汇总 (token 数: {}) ====", by_token.len());
    for (i, kind) in KINDS.iter().enumerate() {
        println!("[{kind}] {}", fmt(&totals[i]));
    }
}
