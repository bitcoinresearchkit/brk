use brk_error::Result;
use brk_types::{Close, High, Low, OHLCSats, Open, Sats};
use vecdb::Exit;

use super::Vecs;
use super::super::usd;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        usd: &usd::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Chain indexes in sats (1 BTC / price)
        self.chainindexes_to_price_open_in_sats
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &usd.chainindexes_to_price_open.height,
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_high_in_sats
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &usd.chainindexes_to_price_low.height,
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_low_in_sats
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &usd.chainindexes_to_price_high.height,
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_close_in_sats
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &usd.chainindexes_to_price_close.height,
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )?;
                Ok(())
            })?;

        // Time indexes in sats
        self.timeindexes_to_price_open_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.timeindexes_to_price_open.dateindex,
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_high_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.timeindexes_to_price_low.dateindex,
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_low_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.timeindexes_to_price_high.dateindex,
                    |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_close_in_sats
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.timeindexes_to_price_close.dateindex,
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )?;
                Ok(())
            })?;

        // Height OHLC in sats
        self.height_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.height,
            &self.chainindexes_to_price_open_in_sats.height,
            &self.chainindexes_to_price_high_in_sats.height,
            &self.chainindexes_to_price_low_in_sats.height,
            &self.chainindexes_to_price_close_in_sats.height,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        // DateIndex OHLC in sats
        self.dateindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.dateindex,
            &self.timeindexes_to_price_open_in_sats.dateindex,
            &self.timeindexes_to_price_high_in_sats.dateindex,
            &self.timeindexes_to_price_low_in_sats.dateindex,
            &self.timeindexes_to_price_close_in_sats.dateindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        // Period OHLC in sats
        self.weekindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.weekindex,
            &*self.timeindexes_to_price_open_in_sats.weekindex,
            &*self.timeindexes_to_price_high_in_sats.weekindex,
            &*self.timeindexes_to_price_low_in_sats.weekindex,
            &*self.timeindexes_to_price_close_in_sats.weekindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.difficultyepoch_to_price_ohlc_in_sats
            .compute_transform4(
                starting_indexes.difficultyepoch,
                &*self.chainindexes_to_price_open_in_sats.difficultyepoch,
                &*self.chainindexes_to_price_high_in_sats.difficultyepoch,
                &*self.chainindexes_to_price_low_in_sats.difficultyepoch,
                &*self.chainindexes_to_price_close_in_sats.difficultyepoch,
                |(i, open, high, low, close, _)| {
                    (
                        i,
                        OHLCSats {
                            open,
                            high,
                            low,
                            close,
                        },
                    )
                },
                exit,
            )?;

        self.monthindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.monthindex,
            &*self.timeindexes_to_price_open_in_sats.monthindex,
            &*self.timeindexes_to_price_high_in_sats.monthindex,
            &*self.timeindexes_to_price_low_in_sats.monthindex,
            &*self.timeindexes_to_price_close_in_sats.monthindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.quarterindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.quarterindex,
            &*self.timeindexes_to_price_open_in_sats.quarterindex,
            &*self.timeindexes_to_price_high_in_sats.quarterindex,
            &*self.timeindexes_to_price_low_in_sats.quarterindex,
            &*self.timeindexes_to_price_close_in_sats.quarterindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.semesterindex_to_price_ohlc_in_sats
            .compute_transform4(
                starting_indexes.semesterindex,
                &*self.timeindexes_to_price_open_in_sats.semesterindex,
                &*self.timeindexes_to_price_high_in_sats.semesterindex,
                &*self.timeindexes_to_price_low_in_sats.semesterindex,
                &*self.timeindexes_to_price_close_in_sats.semesterindex,
                |(i, open, high, low, close, _)| {
                    (
                        i,
                        OHLCSats {
                            open,
                            high,
                            low,
                            close,
                        },
                    )
                },
                exit,
            )?;

        self.yearindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.yearindex,
            &*self.timeindexes_to_price_open_in_sats.yearindex,
            &*self.timeindexes_to_price_high_in_sats.yearindex,
            &*self.timeindexes_to_price_low_in_sats.yearindex,
            &*self.timeindexes_to_price_close_in_sats.yearindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.decadeindex_to_price_ohlc_in_sats.compute_transform4(
            starting_indexes.decadeindex,
            &*self.timeindexes_to_price_open_in_sats.decadeindex,
            &*self.timeindexes_to_price_high_in_sats.decadeindex,
            &*self.timeindexes_to_price_low_in_sats.decadeindex,
            &*self.timeindexes_to_price_close_in_sats.decadeindex,
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCSats {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        Ok(())
    }
}
