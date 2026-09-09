use brk_error::Result;

use bitview_cohort::{SpendableType, SpendableTypeId};
use bitview_plugin_indexer::Indexer;
use brk_error::OptionData;
use brk_exit::Exit;
use brk_types::{StoredU16, StoredU64};
use vecdb::{AnyStoredVec, AnyVec, ReadableVec, VecIndex, WritableVec};

use super::Vecs;
use bitview_compute::{CoinbasePolicy, walk_blocks};

const WRITE_INTERVAL: usize = 10_000;

impl Vecs {
    pub fn compute(&mut self, indexer: &Indexer, exit: &Exit) -> Result<()> {
        let starting_lengths = indexer.safe_lengths();

        let dep_version = indexer.vecs().inputs.output_type.version()
            + indexer.vecs().transactions.first_tx_index.version()
            + indexer.vecs().transactions.first_txin_index.version()
            + indexer.vecs().transactions.txid.version();

        for target in self.stored_vecs_mut() {
            target.any_validate_computed_version_or_reset(dep_version)?;
            target.any_truncate_if_needed_at(starting_lengths.height.to_usize())?;
        }
        let skip = self
            .stored_vecs_mut()
            .map(|target| target.len())
            .min()
            .unwrap_or_default();

        let first_tx_index = &indexer.vecs().transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip < end {
            for target in self.stored_vecs_mut() {
                target.any_truncate_if_needed_at(skip)?;
            }
            let mut cumulative = SpendableType::from_fn(|id| {
                id.select(&self.tx_count_stored)
                    .collect_last()
                    .unwrap_or_default()
            });

            let fi_batch = first_tx_index.collect_range_at(skip, end);
            let txid_len = indexer.vecs().transactions.txid.len();
            let total_txin_len = indexer.vecs().inputs.output_type.len();

            let mut fi_in_cursor = indexer.vecs().transactions.first_txin_index.cursor();
            let first_tx = fi_batch
                .first()
                .expect("block range is nonempty")
                .to_usize()
                + 1;
            let first_txin = if first_tx < txid_len {
                fi_in_cursor.get(first_tx).data()?.to_usize()
            } else {
                total_txin_len
            };
            let mut itype_cursor = indexer
                .vecs()
                .inputs
                .output_type
                .range_cursor_at(first_txin, total_txin_len);
            let mut height = skip;

            walk_blocks(
                &fi_batch,
                txid_len,
                CoinbasePolicy::Skip,
                |tx_pos, per_tx| {
                    let fi_in = fi_in_cursor.get(tx_pos).data()?.to_usize();
                    let next_fi_in = if tx_pos + 1 < txid_len {
                        fi_in_cursor.get(tx_pos + 1).data()?.to_usize()
                    } else {
                        total_txin_len
                    };

                    itype_cursor.advance(fi_in - itype_cursor.position());
                    itype_cursor.for_each(next_fi_in - fi_in, |otype| {
                        per_tx[otype as usize] += 1;
                    });
                    Ok(())
                },
                |agg| {
                    for &id in SpendableTypeId::ALL {
                        let output_type = id.output_type();
                        let value = agg.entries_per_type[output_type as usize];
                        debug_assert!(u16::try_from(value).is_ok());
                        self.input_count_stored
                            .get_mut(output_type)
                            .push(StoredU16::new(value as u16));
                        let total = cumulative.get_mut(output_type);
                        *total += StoredU64::from(agg.txs_per_type[output_type as usize]);
                        self.tx_count_stored.get_mut(output_type).push(*total);
                    }

                    height += 1;
                    if height.is_multiple_of(WRITE_INTERVAL) {
                        let _lock = exit.lock();
                        for target in self.stored_vecs_mut() {
                            target.write()?;
                        }
                    }
                    Ok(())
                },
            )?;

            {
                let _lock = exit.lock();
                for target in self.stored_vecs_mut() {
                    target.write()?;
                }
            }
        }

        Ok(())
    }
}

impl Vecs {
    fn stored_vecs_mut(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        self.input_count_stored
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .chain(
                self.tx_count_stored
                    .iter_mut()
                    .map(|v| v as &mut dyn AnyStoredVec),
            )
    }
}
