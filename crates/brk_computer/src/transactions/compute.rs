use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::{Vecs, type_counts::compute_type_percents};
use crate::{blocks, indexes, inputs, outputs, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        inputs: &inputs::Vecs,
        outputs: &outputs::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let (r1, (r2, (r3, (r4, r5)))) = rayon::join(
            || {
                self.count
                    .compute(indexer, &blocks.lookback, starting_indexes, exit)
            },
            || {
                rayon::join(
                    || self.versions.compute(indexer, starting_indexes, exit),
                    || {
                        rayon::join(
                            || self.size.compute(indexer, indexes, starting_indexes, exit),
                            || {
                                rayon::join(
                                    || {
                                        self.input_types
                                            .compute(indexer, starting_indexes, exit)
                                    },
                                    || {
                                        self.output_types
                                            .compute(indexer, starting_indexes, exit)
                                    },
                                )
                            },
                        )
                    },
                )
            },
        );
        r1?;
        r2?;
        r3?;
        r4?;
        r5?;

        let count_total = &self.count.total;
        let (input_types, output_types) = (&mut self.input_types, &mut self.output_types);
        let (r6, r7) = rayon::join(
            || {
                compute_type_percents(
                    &input_types.by_type,
                    &mut input_types.percent,
                    count_total,
                    starting_indexes.height,
                    exit,
                )
            },
            || {
                compute_type_percents(
                    &output_types.by_type,
                    &mut output_types.percent,
                    count_total,
                    starting_indexes.height,
                    exit,
                )
            },
        );
        r6?;
        r7?;

        self.fees.compute(
            indexer,
            indexes,
            &inputs.spent,
            &self.size,
            starting_indexes,
            exit,
        )?;

        self.volume.compute(
            indexer,
            indexes,
            prices,
            &self.count,
            &self.fees,
            &inputs.count,
            &outputs.count,
            starting_indexes,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
