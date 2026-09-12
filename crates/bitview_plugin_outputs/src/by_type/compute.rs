use bitview_cohort::ByType;
use bitview_compute::{CoinbasePolicy, walk_blocks};
use bitview_plugin_indexer::Indexer;
use brk_error::{OptionData, Result};
use brk_exit::Exit;
use brk_types::StoredU64;
use vecdb::{AnyStoredVec, AnyVec, ReadableVec, VecIndex, WritableVec};

use super::Vecs;

const WRITE_INTERVAL: usize = 10_000;

pub fn compute(vecs: &mut Vecs, indexer: &Indexer, exit: &Exit) -> Result<()> {
    let starting_lengths = indexer.safe_lengths();

    let dep_version = indexer.vecs().outputs.output_type.version()
        + indexer.vecs().transactions.first_tx_index.version()
        + indexer.vecs().transactions.first_txout_index.version()
        + indexer.vecs().transactions.txid.version();

    for target in vecs.stored_vecs_mut() {
        target.any_validate_computed_version_or_reset(dep_version)?;
        target.any_truncate_if_needed_at(starting_lengths.height.to_usize())?;
    }
    let skip = vecs
        .stored_vecs_mut()
        .map(|target| target.len())
        .min()
        .unwrap_or_default();

    let first_tx_index = &indexer.vecs().transactions.first_tx_index;
    let end = first_tx_index.len();
    if skip < end {
        for target in vecs.stored_vecs_mut() {
            target.any_truncate_if_needed_at(skip)?;
        }
        let mut output_cumulative = ByType::from_fn(|output_type| {
            vecs.output_count_stored
                .get(output_type)
                .collect_last()
                .unwrap_or_default()
        });
        let mut tx_cumulative = ByType::from_fn(|output_type| {
            vecs.tx_count_stored
                .get(output_type)
                .collect_last()
                .unwrap_or_default()
        });

        let fi_batch = first_tx_index.collect_range_at(skip, end);
        let txid_len = indexer.vecs().transactions.txid.len();
        let total_txout_len = indexer.vecs().outputs.output_type.len();
        let first_txout_index = &indexer.vecs().transactions.first_txout_index;
        let first_tx = fi_batch
            .first()
            .expect("block range is nonempty")
            .to_usize();
        let mut first_txout_cursor = first_txout_index.range_cursor_at(first_tx, txid_len);
        let first_txout = first_txout_cursor.next().data()?.to_usize();
        let mut output_type_cursor = indexer
            .vecs()
            .outputs
            .output_type
            .range_cursor_at(first_txout, total_txout_len);
        let mut height = skip;

        walk_blocks(
            &fi_batch,
            txid_len,
            CoinbasePolicy::Include,
            |tx_pos, per_tx| {
                let next_first_txout = if tx_pos + 1 < txid_len {
                    first_txout_cursor.next().data()?.to_usize()
                } else {
                    total_txout_len
                };

                let output_count = next_first_txout - output_type_cursor.position();
                output_type_cursor.for_each(output_count, |otype| {
                    per_tx[otype as usize] += 1;
                });
                Ok(())
            },
            |agg| {
                for (output_type, target) in vecs.output_count_stored.iter_typed_mut() {
                    let total = output_cumulative.get_mut(output_type);
                    *total += StoredU64::from(agg.entries_per_type[output_type as usize]);
                    target.push(*total);
                    let total = tx_cumulative.get_mut(output_type);
                    *total += StoredU64::from(agg.txs_per_type[output_type as usize]);
                    vecs.tx_count_stored.get_mut(output_type).push(*total);
                }

                height += 1;
                if height.is_multiple_of(WRITE_INTERVAL) {
                    let _lock = exit.lock();
                    for target in vecs.stored_vecs_mut() {
                        target.write()?;
                    }
                }
                Ok(())
            },
        )?;

        {
            let _lock = exit.lock();
            for target in vecs.stored_vecs_mut() {
                target.write()?;
            }
        }
    }

    Ok(())
}

impl Vecs {
    fn stored_vecs_mut(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        self.output_count_stored
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .chain(
                self.tx_count_stored
                    .iter_mut()
                    .map(|v| v as &mut dyn AnyStoredVec),
            )
    }
}
