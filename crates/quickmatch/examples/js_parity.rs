//! Line-protocol oracle for modules/quickmatch-js/test/parity.test.mjs.
//! Input: hex separators, item count, hex items; then tab-separated
//! limit, trigram budget, minimum score, union fallback, hex separators, hex query.
use std::io::{self, BufRead};

use quickmatch::{QuickMatch, QuickMatchConfig};

fn decode(hex: &str) -> String {
    String::from_utf8(
        hex.as_bytes()
            .chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
            .collect(),
    )
    .unwrap()
}

fn separators(hex: &str) -> &'static [char] {
    Box::leak(decode(hex).chars().collect::<Vec<_>>().into_boxed_slice())
}

fn pairs(values: Vec<(u32, u32)>) -> String {
    format!(
        "[{}]",
        values
            .into_iter()
            .map(|(id, count)| format!("[{id},{count}]"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let config = QuickMatchConfig::new().with_separators(separators(&lines.next().unwrap()));
    let count: usize = lines.next().unwrap().parse().unwrap();
    let items = lines
        .by_ref()
        .take(count)
        .map(|line| decode(&line))
        .collect();
    let matcher = QuickMatch::new_owned_with(items, config);
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        let config = QuickMatchConfig::new()
            .with_limit(fields[0].parse().unwrap())
            .with_trigram_budget(fields[1].parse().unwrap())
            .with_min_score(fields[2].parse().unwrap())
            .with_union_fallback(fields[3] == "true")
            .with_separators(separators(fields[4]));
        let query = decode(fields[5]);
        println!(
            "[{},{},{}]",
            pairs(matcher.matches_with_ids_and_matched_words(&query, &config)),
            pairs(matcher.matches_best_with_ids_and_matched_words(&query, &config)),
            pairs(matcher.matches_exact_with_ids_and_matched_words(&query, &config)),
        );
    }
}
