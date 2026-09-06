use std::{borrow::Cow, collections::BTreeMap};

use super::*;

struct Fixture<'a> {
    names: Vec<&'a str>,
    matcher: QuickMatch<'a>,
    descriptions: DescriptionSearch,
}

impl<'a> Fixture<'a> {
    fn new(items: &[(&'a str, &str)]) -> Self {
        let names: Vec<_> = items.iter().map(|(name, _)| *name).collect();
        let descriptions = items
            .iter()
            .map(|(name, description)| (*name, Cow::Owned((*description).to_owned())))
            .collect::<BTreeMap<_, _>>();
        Self {
            matcher: QuickMatch::new(&names),
            descriptions: DescriptionSearch::new(&names, &descriptions),
            names,
        }
    }

    fn search(&self, query: &str, limit: usize) -> Vec<&'a str> {
        matches(&self.names, &self.matcher, &self.descriptions, query, limit)
    }
}

#[test]
fn exact_names_then_exact_descriptions_then_fuzzy_names_then_fuzzy_descriptions() {
    let fixture = Fixture::new(&[
        ("epsilon", "alpha betamax"),
        ("alpha_betamax", "unrelated"),
        ("delta", "alpha beta"),
        ("alpha_beta", "unrelated"),
    ]);
    let expected = ["alpha_beta", "delta", "alpha_betamax", "epsilon"];
    for query in ["alpha beta", "beta alpha", "ALPHA_BETA"] {
        for limit in 0..=6 {
            assert_eq!(
                fixture.search(query, limit),
                expected[..limit.min(expected.len())]
            );
        }
    }
}

#[test]
fn descriptions_supply_cohort_words_without_alias_expansion() {
    let fixture = Fixture::new(&[
        (
            "sth_mvrv",
            "Short-term-holder ratio of spot price to realized price.",
        ),
        (
            "sth_realized_price",
            "Realized price of short-term-holder outputs.",
        ),
        (
            "lth_realized_price",
            "Realized price of long-term-holder outputs.",
        ),
        ("realized_price", "Realized price of all outputs."),
    ]);
    for query in [
        "realized price short term",
        "short term holder realized price",
        "realized prcie sth",
    ] {
        assert_eq!(
            fixture.search(query, 10)[0],
            "sth_realized_price",
            "{query}"
        );
    }
    assert_eq!(
        fixture.search("long term realized price", 10)[0],
        "lth_realized_price"
    );
}

#[test]
fn empty_queries_and_duplicate_words_are_stable() {
    let fixture = Fixture::new(&[("price", "A price."), ("price_ratio", "Ratio of prices.")]);
    assert!(fixture.search("", 10).is_empty());
    assert!(fixture.search("...", 10).is_empty());
    assert!(fixture.search("price", 0).is_empty());
    assert_eq!(
        fixture.search("price price", 10),
        fixture.search("price", 10)
    );
    assert!(fixture.search("zqxwvv", 10).is_empty());
}

#[test]
fn limited_results_preserve_ranking_with_duplicates_across_tiers() {
    let names = (0..100)
        .map(|id| format!("metric_{id}"))
        .collect::<Vec<_>>();
    let mut items = names
        .iter()
        .map(|name| (name.as_str(), "alpha beta"))
        .collect::<Vec<_>>();
    items.push(("alpha_beta", "alpha beta"));
    items.push(("alpha_betamax", "alpha beta"));
    let fixture = Fixture::new(&items);
    let all = fixture.search("alpha beta", items.len());
    assert_eq!(all.len(), items.len());
    for limit in [0, 1, 2, 3, 10, 50, 101, 102, 103] {
        assert_eq!(
            fixture.search("alpha beta", limit),
            all[..limit.min(all.len())]
        );
    }
}

#[test]
#[ignore = "set SEARCH_CATALOG_SNAPSHOT to a saved /api/series JSON response"]
fn compare_full_catalog_snapshot() {
    let path = std::env::var("SEARCH_CATALOG_SNAPSHOT").unwrap();
    let catalog: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    fn visit<'a>(node: &'a serde_json::Value, items: &mut BTreeMap<&'a str, &'a str>) {
        if let Some(name) = node.get("name").and_then(|v| v.as_str()) {
            items.entry(name).or_insert(
                node.get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or(""),
            );
        } else if let Some(branch) = node.as_object() {
            for node in branch.values() {
                visit(node, items);
            }
        }
    }
    let mut items = BTreeMap::new();
    visit(&catalog, &mut items);
    let fixture = Fixture::new(&items.into_iter().collect::<Vec<_>>());
    let queries = [
        "realized price sth",
        "short term holder realized price",
        "realized price short term",
        "realized prcie sth",
        "prcie",
        "profit supply sth",
        "short term supply in profit",
        "short term mvrv",
        "hashrate",
        "hashraet",
        "hash rte",
        "cap market",
        "bitcoin price",
        "average transaction fee",
        "active addresses",
        "coin days destroyed",
        "cdd",
        "long term holder supply",
        "unrealized profit loss",
    ];
    let mut timings = Vec::new();
    for _ in 0..20 {
        for query in queries {
            let start = std::time::Instant::now();
            std::hint::black_box(fixture.search(query, 50));
            timings.push((start.elapsed().as_secs_f64() * 1000.0, query));
        }
    }
    timings.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!(
        "TIMING median={:?} p95={:?} max={:?}",
        timings[timings.len() / 2],
        timings[timings.len() * 95 / 100],
        timings.last()
    );
    for query in queries {
        println!(
            "{}",
            serde_json::json!({"query": query, "top": fixture.search(query, 5)})
        );
    }
    for (query, expected) in [
        ("realized price sth", "sth_realized_price"),
        ("short term holder realized price", "sth_realized_price"),
        ("realized price short term", "sth_realized_price"),
        ("realized prcie sth", "sth_realized_price"),
        ("prcie", "price"),
        ("hashrate", "hash_rate"),
        ("hashraet", "hash_rate"),
        ("profit supply sth", "sth_supply_in_profit"),
        ("cap market", "market_cap"),
        ("active addresses", "active_addrs"),
        ("coin days destroyed", "coindays_destroyed"),
    ] {
        assert_eq!(fixture.search(query, 5)[0], expected, "{query}");
    }
}
