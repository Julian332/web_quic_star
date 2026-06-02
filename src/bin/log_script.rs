use std::fs;

fn main() {
    let path = "src/bin/watcher_20260531_1x.log";
    let threshold = "2026-05-31 04:34:50.324";
    let content = fs::read_to_string(path).unwrap();
    let filtered = content
        .lines()
        .filter(|line| {
            line.get(..threshold.len())
                .map_or(false, |ts| ts <= threshold)
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(path, filtered).unwrap();
}
