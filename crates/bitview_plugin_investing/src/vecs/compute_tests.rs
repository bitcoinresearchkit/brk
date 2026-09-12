use bitview_vecs::{LazyPreviousDeltaVec, import_cached};
use brk_types::{Cents, Height, Version};
use tempfile::tempdir;
use vecdb::{AnyStoredVec, Budgeted, Database, LazyVec, ReadableCloneableVec, WritableVec};

use super::*;
use crate::dca_sats::DcaSats;

#[test]
fn daily_totals_survive_append_rewrite_shrink_eviction_and_reopen() {
    let budget = Budgeted::init_global(16 * 1024 * 1024).unwrap();
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut closes = import_cached::<Day1, Cents>(&db, "closes", Version::ONE).unwrap();
    let prices = LazyVec::init(
        "prices",
        Version::ONE,
        closes.read_only_boxed_clone(),
        |_, price: Cents| (!price.is_nan()).then(|| Dollars::from(price)),
    );
    let mut totals = import_cached::<Day1, Sats>(&db, "totals", Version::ONE).unwrap();
    let mut days = import_cached::<Height, Day1>(&db, "days", Version::ONE).unwrap();
    // Repeated days and a skipped day exercise the block deltas used by DCA views.
    let expected_days: Vec<_> = (0..4300)
        .flat_map(|i| [i, i])
        .filter(|&i| i != 1024)
        .collect();
    for &day in &expected_days {
        days.push(Day1::from(day));
    }
    days.write().unwrap();
    let reader = totals.read_only_boxed_clone();
    let mapped = DcaSats::new(reader.clone(), days.read_only_boxed_clone());
    let deltas = LazyPreviousDeltaVec::new("deltas", Version::ONE, &mapped);
    let exit = Exit::default();
    let mut expected_prices = Vec::new();

    for (from, end, offset) in [
        (0, 4096, 0),
        (4096, 4300, 0),
        (1023, 4300, 17),
        (2000, 2000, 0),
        (0, 4300, 31),
        (0, 0, 0),
        (0, 4300, 0),
    ] {
        closes.truncate_if_needed_at(from).unwrap();
        expected_prices.truncate(from);
        for i in from..end {
            let price = match i % 7 {
                0 => Cents::NAN,
                1 => Cents::ZERO,
                _ => Cents::new(10_000 + ((i * 37 + offset) % 1000) as u64),
            };
            closes.push(price);
            expected_prices.push(price);
        }
        closes.write().unwrap();
        compute_daily_sats(&mut totals, Day1::from(from), &prices, &exit).unwrap();

        let mut sum = Sats::ZERO;
        let expected_totals = expected_prices
            .iter()
            .map(|&price| {
                sum += Sats::from_dollars_at_price(DCA_AMOUNT, Dollars::from(price));
                sum
            })
            .collect::<Vec<_>>();
        let expected: Vec<_> = expected_days
            .iter()
            .map(|&day| {
                end.checked_sub(1)
                    .map(|last| expected_totals[day.min(last)])
                    .unwrap_or_default()
            })
            .collect();
        let mut before = Sats::ZERO;
        let expected_deltas: Vec<_> = expected
            .iter()
            .map(|&total| {
                let delta = total - before;
                before = total;
                delta
            })
            .collect();

        for cold in [false, true] {
            if cold {
                budget.clear();
            }
            assert_eq!(reader.collect(), expected_totals);
            assert_eq!(mapped.collect(), expected);
            assert_eq!(deltas.collect(), expected_deltas);
            assert_eq!(
                mapped.collect_one_at(expected.len() - 1),
                expected.last().copied()
            );
            let indices = [0, 255, 256, 256, expected.len() - 1, expected.len()];
            assert_eq!(
                mapped.read_sorted_at(&indices),
                indices
                    .iter()
                    .filter_map(|&i| expected.get(i).copied())
                    .collect::<Vec<_>>()
            );
            assert_eq!(mapped.collect_range_at(250, 270), expected[250..270]);
        }
    }
    // A source-version change rebuilds even when the requested start is the tip.
    let revised = LazyVec::init(
        "prices",
        Version::TWO,
        closes.read_only_boxed_clone(),
        |_, _| Some(Dollars::mint(200.0)),
    );
    compute_daily_sats(
        &mut totals,
        Day1::from(expected_prices.len()),
        &revised,
        &exit,
    )
    .unwrap();
    let expected_totals: Vec<_> = (1..=expected_prices.len())
        .map(|days| Sats::from(50_000_000 * days as u64))
        .collect();
    assert_eq!(reader.collect(), expected_totals);
    assert_eq!(
        mapped.collect_one_at(expected_days.len() - 1),
        expected_totals.last().copied()
    );

    drop((
        deltas, mapped, reader, days, revised, prices, closes, totals, db,
    ));
    let db = Database::open(directory.path()).unwrap();
    let totals = import_cached::<Day1, Sats>(&db, "totals", Version::ONE).unwrap();
    assert_eq!(totals.collect(), expected_totals);
}
