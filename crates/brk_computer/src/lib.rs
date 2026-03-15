#![doc = include_str!("../README.md")]

use std::{fs, path::Path, thread, time::Instant};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_traversable::Traversable;
use brk_types::Version;
use tracing::info;
use vecdb::{AnyExportableVec, Exit, Ro, Rw, StorageMode};

mod blocks;
mod cointime;
mod constants;
mod distribution;
mod indicators;
pub mod indexes;
mod inputs;
mod internal;
mod market;
mod mining;
mod outputs;
mod pools;
mod positions;
pub mod prices;
mod scripts;
mod supply;
mod transactions;

#[derive(Traversable)]
pub struct Computer<M: StorageMode = Rw> {
    pub blocks: Box<blocks::Vecs<M>>,
    pub mining: Box<mining::Vecs<M>>,
    pub transactions: Box<transactions::Vecs<M>>,
    pub scripts: Box<scripts::Vecs<M>>,
    #[traversable(hidden)]
    pub positions: Box<positions::Vecs<M>>,
    pub cointime: Box<cointime::Vecs<M>>,
    pub constants: Box<constants::Vecs>,
    pub indexes: Box<indexes::Vecs<M>>,
    pub indicators: Box<indicators::Vecs<M>>,
    pub market: Box<market::Vecs<M>>,
    pub pools: Box<pools::Vecs<M>>,
    pub prices: Box<prices::Vecs<M>>,
    #[traversable(flatten)]
    pub distribution: Box<distribution::Vecs<M>>,
    pub supply: Box<supply::Vecs<M>>,
    pub inputs: Box<inputs::Vecs<M>>,
    pub outputs: Box<outputs::Vecs<M>>,
}

const VERSION: Version = Version::new(5);

impl Computer {
    pub fn forced_import(outputs_path: &Path, indexer: &Indexer) -> Result<Self> {
        info!("Importing computer...");
        let import_start = Instant::now();

        let computed_path = outputs_path.join("computed");

        const STACK_SIZE: usize = 8 * 1024 * 1024;
        let big_thread = || thread::Builder::new().stack_size(STACK_SIZE);

        let (indexes, positions) = timed("Imported indexes/positions", || {
            thread::scope(|s| -> Result<_> {
                let positions_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                    Ok(Box::new(positions::Vecs::forced_import(
                        &computed_path,
                        VERSION,
                    )?))
                })?;

                let indexes = Box::new(indexes::Vecs::forced_import(
                    &computed_path,
                    VERSION,
                    indexer,
                )?);
                let positions = positions_handle.join().unwrap()?;

                Ok((indexes, positions))
            })
        })?;

        let (constants, prices) = timed("Imported prices/constants", || -> Result<_> {
            let constants = Box::new(constants::Vecs::new(VERSION, &indexes));
            let prices = Box::new(prices::Vecs::forced_import(
                &computed_path,
                VERSION,
                &indexes,
            )?);
            Ok((constants, prices))
        })?;

        let blocks = timed("Imported blocks", || -> Result<_> {
            Ok(Box::new(blocks::Vecs::forced_import(
                &computed_path,
                VERSION,
                indexer,
                &indexes,
            )?))
        })?;

        let cached_starts = &blocks.lookback.cached_window_starts;

        let (inputs, outputs, mining, transactions, scripts, pools, cointime) =
            timed("Imported inputs/outputs/mining/tx/scripts/pools/cointime", || {
                thread::scope(|s| -> Result<_> {
                    let inputs_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(inputs::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let outputs_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(outputs::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let mining_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(mining::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let transactions_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(transactions::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            indexer,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let scripts_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(scripts::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let pools_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(pools::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                            cached_starts,
                        )?))
                    })?;

                    let cointime = Box::new(cointime::Vecs::forced_import(
                        &computed_path,
                        VERSION,
                        &indexes,
                        cached_starts,
                    )?);

                    let inputs = inputs_handle.join().unwrap()?;
                    let outputs = outputs_handle.join().unwrap()?;
                    let mining = mining_handle.join().unwrap()?;
                    let transactions = transactions_handle.join().unwrap()?;
                    let scripts = scripts_handle.join().unwrap()?;
                    let pools = pools_handle.join().unwrap()?;

                    Ok((inputs, outputs, mining, transactions, scripts, pools, cointime))
                })
            })?;

        // Market, indicators, and distribution are independent; import in parallel.
        // Supply depends on distribution so it runs after.
        let (distribution, market, indicators) =
            timed("Imported distribution/market/indicators", || {
                thread::scope(|s| -> Result<_> {
                    let market_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(market::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                        )?))
                    })?;

                    let indicators_handle = big_thread().spawn_scoped(s, || -> Result<_> {
                        Ok(Box::new(indicators::Vecs::forced_import(
                            &computed_path,
                            VERSION,
                            &indexes,
                        )?))
                    })?;

                    let distribution = Box::new(distribution::Vecs::forced_import(
                        &computed_path,
                        VERSION,
                        &indexes,
                        cached_starts,
                    )?);

                    let market = market_handle.join().unwrap()?;
                    let indicators = indicators_handle.join().unwrap()?;
                    Ok((distribution, market, indicators))
                })
            })?;

        let supply = timed("Imported supply", || -> Result<_> {
            Ok(Box::new(supply::Vecs::forced_import(
                &computed_path,
                VERSION,
                &indexes,
                &distribution,
                &cointime,
                cached_starts,
            )?))
        })?;

        info!("Total import time: {:?}", import_start.elapsed());

        let this = Self {
            blocks,
            mining,
            transactions,
            scripts,
            constants,
            indicators,
            market,
            distribution,
            supply,
            positions,
            pools,
            cointime,
            indexes,
            inputs,
            prices,
            outputs,
        };

        Self::retain_databases(&computed_path)?;

        Ok(this)
    }

    /// Removes database folders that are no longer in use.
    fn retain_databases(computed_path: &Path) -> Result<()> {
        const EXPECTED_DBS: &[&str] = &[
            blocks::DB_NAME,
            mining::DB_NAME,
            transactions::DB_NAME,
            scripts::DB_NAME,
            positions::DB_NAME,
            cointime::DB_NAME,
            indicators::DB_NAME,
            indexes::DB_NAME,
            market::DB_NAME,
            pools::DB_NAME,
            prices::DB_NAME,
            distribution::DB_NAME,
            supply::DB_NAME,
            inputs::DB_NAME,
            outputs::DB_NAME,
        ];

        if !computed_path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(computed_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if !file_type.is_dir() {
                continue;
            }

            if let Some(name) = entry.file_name().to_str()
                && !EXPECTED_DBS.contains(&name)
            {
                info!("Removing obsolete database folder: {}", name);
                fs::remove_dir_all(entry.path())?;
            }
        }

        Ok(())
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        reader: &Reader,
        exit: &Exit,
    ) -> Result<()> {
        let compute_start = Instant::now();

        let mut starting_indexes = timed("Computed indexes", || {
            self.indexes
                .compute(indexer, &mut self.blocks, starting_indexes, exit)
        })?;

        timed("Computed prices", || {
            self.prices
                .compute(indexer, &self.indexes, &starting_indexes, exit)
        })?;

        thread::scope(|scope| -> Result<()> {
            let positions = scope.spawn(|| {
                timed("Computed positions", || {
                    self.positions
                        .compute(indexer, &starting_indexes, reader, exit)
                })
            });

            timed("Computed blocks", || {
                self.blocks
                    .compute(indexer, &self.indexes, &starting_indexes, exit)
            })?;

            // inputs and scripts are independent — parallelize
            let (inputs_result, scripts_result) = rayon::join(
                || {
                    timed("Computed inputs", || {
                        self.inputs.compute(
                            indexer,
                            &self.indexes,
                            &self.blocks,
                            &starting_indexes,
                            exit,
                        )
                    })
                },
                || {
                    timed("Computed scripts", || {
                        self.scripts.compute(
                            indexer,
                            &self.outputs,
                            &self.prices,
                            &starting_indexes,
                            exit,
                        )
                    })
                },
            );
            inputs_result?;
            scripts_result?;

            timed("Computed outputs", || {
                self.outputs.compute(
                    indexer,
                    &self.indexes,
                    &self.inputs,
                    &self.scripts,
                    &self.blocks,
                    &starting_indexes,
                    exit,
                )
            })?;

            timed("Computed transactions", || {
                self.transactions.compute(
                    indexer,
                    &self.indexes,
                    &self.blocks,
                    &self.inputs,
                    &self.outputs,
                    &self.prices,
                    &starting_indexes,
                    exit,
                )
            })?;

            timed("Computed mining", || {
                self.mining.compute(
                    indexer,
                    &self.indexes,
                    &self.blocks,
                    &self.transactions,
                    &self.prices,
                    &starting_indexes,
                    exit,
                )
            })?;

            positions.join().unwrap()?;
            Ok(())
        })?;

        let starting_indexes_clone = starting_indexes.clone();
        thread::scope(|scope| -> Result<()> {
            let pools = scope.spawn(|| {
                timed("Computed pools", || {
                    self.pools.compute(
                        indexer,
                        &self.indexes,
                        &self.blocks,
                        &self.prices,
                        &self.mining,
                        &starting_indexes_clone,
                        exit,
                    )
                })
            });

            timed("Computed distribution", || {
                self.distribution.compute(
                    indexer,
                    &self.indexes,
                    &self.inputs,
                    &self.outputs,
                    &self.transactions,
                    &self.blocks,
                    &self.prices,
                    &mut starting_indexes,
                    exit,
                )
            })?;

            pools.join().unwrap()?;
            Ok(())
        })?;

        thread::scope(|scope| -> Result<()> {
            let market = scope.spawn(|| {
                timed("Computed market", || {
                    self.market.compute(
                        &self.indexes,
                        &self.prices,
                        &self.blocks,
                        &starting_indexes,
                        exit,
                    )
                })
            });

            timed("Computed supply", || {
                self.supply.compute(
                    &self.scripts,
                    &self.blocks,
                    &self.mining,
                    &self.transactions,
                    &self.prices,
                    &self.distribution,
                    &starting_indexes,
                    exit,
                )
            })?;

            market.join().unwrap()?;
            Ok(())
        })?;

        thread::scope(|scope| -> Result<()> {
            let indicators = scope.spawn(|| {
                timed("Computed indicators", || {
                    self.indicators.compute(
                        &self.mining,
                        &self.distribution,
                        &self.transactions,
                        &self.market,
                        &starting_indexes,
                        exit,
                    )
                })
            });

            timed("Computed cointime", || {
                self.cointime.compute(
                    &starting_indexes,
                    &self.prices,
                    &self.blocks,
                    &self.mining,
                    &self.supply,
                    &self.distribution,
                    exit,
                )
            })?;

            indicators.join().unwrap()?;
            Ok(())
        })?;

        info!("Total compute time: {:?}", compute_start.elapsed());
        Ok(())
    }
}

macro_rules! impl_iter_named_exportable {
    ($($field:ident),+ $(,)?) => {
        impl_iter_named_exportable!(@mode Ro, $($field),+);
        impl_iter_named_exportable!(@mode Rw, $($field),+);
    };
    (@mode $mode:ty, $($field:ident),+) => {
        impl Computer<$mode> {
            pub fn iter_named_exportable(
                &self,
            ) -> impl Iterator<Item = (&'static str, &dyn AnyExportableVec)> {
                use brk_traversable::Traversable;
                std::iter::empty()
                    $(.chain(self.$field.iter_any_exportable().map(|v| ($field::DB_NAME, v))))+
            }
        }
    };
}

impl_iter_named_exportable!(blocks, mining, transactions, scripts, positions, cointime,
    constants, indicators, indexes, market, pools, prices, distribution, supply, inputs, outputs);

fn timed<T>(label: &str, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    info!("{label} in {:?}", start.elapsed());
    result
}
