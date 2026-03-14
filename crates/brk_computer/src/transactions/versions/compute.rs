use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64, TxVersion};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = indexer.vecs.transactions.tx_version.version()
            + indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.txid.version();

        for vec in [
            &mut self.v1.base.height,
            &mut self.v2.base.height,
            &mut self.v3.base.height,
        ] {
            vec.validate_and_truncate(dep_version, starting_indexes.height)?;
        }

        let skip = self
            .v1
            .base
            .height
            .len()
            .min(self.v2.base.height.len())
            .min(self.v3.base.height.len());

        let first_tx_index = &indexer.vecs.transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip >= end {
            return Ok(());
        }

        // Truncate all 3 to skip, then push (no per-element bounds checks).
        self.v1.base.height.truncate_if_needed_at(skip)?;
        self.v2.base.height.truncate_if_needed_at(skip)?;
        self.v3.base.height.truncate_if_needed_at(skip)?;

        // Single cursor over tx_version — scanned once for all 3 version counts.
        let mut cursor = indexer.vecs.transactions.tx_version.cursor();
        let fi_batch = first_tx_index.collect_range_at(skip, end);
        let txid_len = indexer.vecs.transactions.txid.len();

        for (j, first_index) in fi_batch.iter().enumerate() {
            let next_first = fi_batch
                .get(j + 1)
                .map(|fi| fi.to_usize())
                .unwrap_or(txid_len);

            let mut c1: usize = 0;
            let mut c2: usize = 0;
            let mut c3: usize = 0;

            let fi = first_index.to_usize();
            cursor.advance(fi - cursor.position());
            for _ in fi..next_first {
                match cursor.next().unwrap() {
                    TxVersion::ONE => c1 += 1,
                    TxVersion::TWO => c2 += 1,
                    TxVersion::THREE => c3 += 1,
                    _ => {}
                }
            }

            self.v1.base.height.push(StoredU64::from(c1 as u64));
            self.v2.base.height.push(StoredU64::from(c2 as u64));
            self.v3.base.height.push(StoredU64::from(c3 as u64));

            if self.v1.base.height.batch_limit_reached() {
                let _lock = exit.lock();
                self.v1.base.height.write()?;
                self.v2.base.height.write()?;
                self.v3.base.height.write()?;
            }
        }

        {
            let _lock = exit.lock();
            self.v1.base.height.write()?;
            self.v2.base.height.write()?;
            self.v3.base.height.write()?;
        }

        // Derive cumulative + sums from base
        self.v1.compute_rest(starting_indexes.height, exit)?;
        self.v2.compute_rest(starting_indexes.height, exit)?;
        self.v3.compute_rest(starting_indexes.height, exit)?;

        Ok(())
    }
}
