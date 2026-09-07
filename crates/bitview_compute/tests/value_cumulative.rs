mod common;

use bitview_compute::{SatsToCents, ValuePerBlockCumulative};
use brk_exit::Exit;
use brk_types::{Cents, Height, Sats, StoredU64, TxIndex, Version};
use vecdb::{BinaryTransform, Database, ReadableVec, WritableVec};

use common::{indexes, stored};

#[test]
fn cumulative_values_reopen_resume_and_rewind_from_stored_totals() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = indexes(&db);
    let length = 16usize;
    let amounts: Vec<_> = (0..length).map(|i| Sats::from((i + 1) as u64)).collect();
    let prices: Vec<_> = (0..length)
        .map(|i| Cents::from((i as u64 + 1) * 100_000_000))
        .collect();
    let source = stored::<Height, _>(&db, "amounts", amounts.iter().copied());
    let price_source = stored::<Height, _>(&db, "prices", prices.iter().copied());
    let mut offset = 0usize;
    let first = stored::<Height, _>(
        &db,
        "first",
        (0..length).map(|i| {
            let start = TxIndex::from(offset);
            offset += i % 4;
            start
        }),
    );
    let counts = stored::<Height, _>(&db, "counts", (0..length).map(|i| StoredU64::from(i % 4)));
    let tx_amounts: Vec<_> = (0..offset)
        .map(|i| {
            if i % 5 == 0 {
                Sats::MAX
            } else {
                Sats::from(i as u64 + 1)
            }
        })
        .collect();
    let tx_source = stored::<TxIndex, _>(&db, "tx_amounts", tx_amounts.iter().copied());

    for mode in 0..3 {
        let name = format!("cumulative_{mode}");
        let mut block_amounts = amounts.clone();
        if mode == 1 {
            block_amounts.iter_mut().for_each(|sats| *sats += *sats);
        } else if mode == 2 {
            let mut offset = 0;
            for (i, amount) in block_amounts.iter_mut().enumerate() {
                let end = offset + i % 4;
                *amount = tx_amounts[offset..end]
                    .iter()
                    .copied()
                    .filter(|sats| !sats.is_max())
                    .sum();
                offset = end;
            }
        }
        let block_cents: Vec<_> = block_amounts
            .iter()
            .copied()
            .zip(prices.iter().copied())
            .map(|(sats, cents)| SatsToCents::apply(sats, cents))
            .collect();
        let mut sum_sats = Sats::ZERO;
        let cumulative_sats: Vec<_> = block_amounts
            .iter()
            .map(|&sats| {
                sum_sats += sats;
                sum_sats
            })
            .collect();
        let mut sum_cents = Cents::ZERO;
        let cumulative_cents: Vec<_> = block_cents
            .iter()
            .map(|&cents| {
                sum_cents += cents;
                sum_cents
            })
            .collect();

        let mut output =
            ValuePerBlockCumulative::forced_import(&db, &name, Version::ONE, &indexes).unwrap();
        for phase in 0..5 {
            if phase == 2 {
                drop(output);
                output = ValuePerBlockCumulative::forced_import(&db, &name, Version::ONE, &indexes)
                    .unwrap();
            }
            if phase == 4 {
                output
                    .cumulative
                    .sats
                    .height
                    .validate_computed_version_or_reset(Version::ZERO)
                    .unwrap();
            }
            let from = Height::from(if phase == 3 { 7usize } else { length });
            let exit = Exit::new();
            match mode {
                0 => output.compute_from(from, &price_source, &source, |_, sats| sats, &exit),
                1 => output.compute_from_pair(
                    from,
                    &price_source,
                    &source,
                    &source,
                    |_, a, b| a + b,
                    &exit,
                ),
                _ => output.compute_filtered_from_indexes(
                    from,
                    &price_source,
                    &first,
                    &counts,
                    &tx_source,
                    |sats| !sats.is_max(),
                    &exit,
                ),
            }
            .unwrap();
            assert_eq!(
                output.cumulative.sats.height.collect_range_at(0, length),
                cumulative_sats,
                "sats mode={mode} phase={phase}"
            );
            assert_eq!(
                output.cumulative.cents.height.collect_range_at(0, length),
                cumulative_cents,
                "cents mode={mode} phase={phase}"
            );
            assert_eq!(
                output.block.sats.collect_range_at(0, length),
                block_amounts,
                "block sats mode={mode} phase={phase}"
            );
            assert_eq!(
                output.block.cents.collect_range_at(0, length),
                block_cents,
                "block cents mode={mode} phase={phase}"
            );
        }
    }
}
