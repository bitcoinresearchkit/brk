use std::{fs, path::Path};

use brk_core::{Dollars, StoredF64, StoredUsize};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation, StoredIndex, VecIterator, Version};

use super::{
    Indexes, fetched,
    grouped::{ComputedVecsFromDateindex, StorableVecGeneatorOptions},
    indexes, transactions,
};

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_marketcap: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_ath: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_drawdown: ComputedVecsFromDateindex<StoredF64>,
    pub indexes_to_days_since_ath: ComputedVecsFromDateindex<StoredUsize>,
    pub indexes_to_max_days_between_ath: ComputedVecsFromDateindex<StoredUsize>,
    pub indexes_to_max_years_between_ath: ComputedVecsFromDateindex<StoredF64>,

    pub indexes_to_1w_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_8d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_13d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_21d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_1m_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_34d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_55d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_89d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_144d_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_1y_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_2y_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_200w_sma: ComputedVecsFromDateindex<Dollars>,
    pub indexes_to_4y_sma: ComputedVecsFromDateindex<Dollars>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            indexes_to_marketcap: ComputedVecsFromDateindex::forced_import(
                path,
                "marketcap",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_ath: ComputedVecsFromDateindex::forced_import(
                path,
                "ath",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_drawdown: ComputedVecsFromDateindex::forced_import(
                path,
                "drawdown",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_days_since_ath: ComputedVecsFromDateindex::forced_import(
                path,
                "days_since_ath",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_max_days_between_ath: ComputedVecsFromDateindex::forced_import(
                path,
                "max_days_between_ath",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_max_years_between_ath: ComputedVecsFromDateindex::forced_import(
                path,
                "max_years_between_ath",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            indexes_to_1w_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "1w_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_8d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "8d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_13d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "13d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_21d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "21d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_1m_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "1m_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_34d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "34d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_55d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "55d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_89d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "89d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_144d_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "144d_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_1y_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "1y_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_2y_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "2y_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_200w_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "200w_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_4y_sma: ComputedVecsFromDateindex::forced_import(
                path,
                "4y_sma",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: &fetched::Vecs,
        transactions: &mut transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_marketcap.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut total_subsidy_in_btc = transactions
                    .indexes_to_subsidy
                    .bitcoin
                    .dateindex
                    .unwrap_total()
                    .into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    |(i, close, ..)| {
                        let supply = total_subsidy_in_btc.unwrap_get_inner(i);
                        (i, *close * supply)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_ath.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_high.dateindex,
                    |(i, high, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                Dollars::ZERO
                            });
                        }
                        let ath = prev.unwrap().max(*high);
                        prev.replace(ath);
                        (i, ath)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_drawdown.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut close_iter = fetched.timeindexes_to_close.dateindex.into_iter();

                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.indexes_to_ath.dateindex,
                    |(i, ath, ..)| {
                        if ath == Dollars::ZERO {
                            return (i, StoredF64::default());
                        }
                        let close = *close_iter.unwrap_get_inner(i);
                        let drawdown = StoredF64::from((*ath - *close) / *ath * -100.0);
                        (i, drawdown)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_days_since_ath.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut high_iter = fetched.timeindexes_to_high.dateindex.into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.indexes_to_ath.dateindex,
                    |(i, ath, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                StoredUsize::default()
                            });
                        }
                        let days = if *high_iter.unwrap_get_inner(i) == ath {
                            StoredUsize::default()
                        } else {
                            prev.unwrap() + StoredUsize::from(1)
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_max_days_between_ath.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.indexes_to_days_since_ath.dateindex,
                    |(i, days, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                StoredUsize::ZERO
                            });
                        }
                        let max = prev.unwrap().max(days);
                        prev.replace(max);
                        (i, max)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_max_years_between_ath.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.indexes_to_max_days_between_ath.dateindex,
                    |(i, max, ..)| (i, StoredF64::from(*max as f64 / 365.0)),
                    exit,
                )
            },
        )?;

        self.indexes_to_1w_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    7,
                    exit,
                )
            },
        )?;

        self.indexes_to_8d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    8,
                    exit,
                )
            },
        )?;

        self.indexes_to_13d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    13,
                    exit,
                )
            },
        )?;

        self.indexes_to_21d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    21,
                    exit,
                )
            },
        )?;

        self.indexes_to_1m_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    30,
                    exit,
                )
            },
        )?;

        self.indexes_to_34d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    34,
                    exit,
                )
            },
        )?;

        self.indexes_to_55d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    55,
                    exit,
                )
            },
        )?;

        self.indexes_to_89d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    89,
                    exit,
                )
            },
        )?;

        self.indexes_to_144d_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    144,
                    exit,
                )
            },
        )?;

        self.indexes_to_1y_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    365,
                    exit,
                )
            },
        )?;

        self.indexes_to_2y_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    2 * 365,
                    exit,
                )
            },
        )?;

        self.indexes_to_200w_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    200 * 7,
                    exit,
                )
            },
        )?;

        self.indexes_to_4y_sma.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    &fetched.timeindexes_to_close.dateindex,
                    4 * 365,
                    exit,
                )
            },
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes_to_marketcap.vecs(),
            self.indexes_to_ath.vecs(),
            self.indexes_to_drawdown.vecs(),
            self.indexes_to_days_since_ath.vecs(),
            self.indexes_to_max_days_between_ath.vecs(),
            self.indexes_to_max_years_between_ath.vecs(),
            self.indexes_to_1w_sma.vecs(),
            self.indexes_to_8d_sma.vecs(),
            self.indexes_to_13d_sma.vecs(),
            self.indexes_to_21d_sma.vecs(),
            self.indexes_to_1m_sma.vecs(),
            self.indexes_to_34d_sma.vecs(),
            self.indexes_to_55d_sma.vecs(),
            self.indexes_to_89d_sma.vecs(),
            self.indexes_to_144d_sma.vecs(),
            self.indexes_to_1y_sma.vecs(),
            self.indexes_to_2y_sma.vecs(),
            self.indexes_to_200w_sma.vecs(),
            self.indexes_to_4y_sma.vecs(),
        ]
        .concat()
    }
}
