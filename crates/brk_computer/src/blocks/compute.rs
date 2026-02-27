use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{ComputeIndexes, indexes};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        {
            let ts = &mut self.time.timestamp;

            macro_rules! period {
                ($field:ident) => {
                    ts.$field.compute_transform(
                        starting_indexes.$field,
                        &indexes.$field.first_height,
                        |(idx, _, _)| (idx, idx.to_timestamp()),
                        exit,
                    )?;
                };
            }

            period!(minute1);
            period!(minute5);
            period!(minute10);
            period!(minute30);
            period!(hour1);
            period!(hour4);
            period!(hour12);
            period!(day1);
            period!(day3);
            period!(week1);
            period!(month1);
            period!(month3);
            period!(month6);
            period!(year1);
            period!(year10);

            ts.halvingepoch.compute_indirect(
                starting_indexes.halvingepoch,
                &indexes.halvingepoch.first_height,
                &indexer.vecs.blocks.timestamp,
                exit,
            )?;
            ts.difficultyepoch.compute_indirect(
                starting_indexes.difficultyepoch,
                &indexes.difficultyepoch.first_height,
                &indexer.vecs.blocks.timestamp,
                exit,
            )?;
        }
        self.count
            .compute(indexer, &self.time, starting_indexes, exit)?;
        self.interval
            .compute(indexer, &self.count, starting_indexes, exit)?;
        self.size.compute(indexer, &self.count, starting_indexes, exit)?;
        self.weight
            .compute(indexer, &self.count, starting_indexes, exit)?;
        self.difficulty
            .compute(indexer, indexes, starting_indexes, exit)?;
        self.halving.compute(indexes, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
