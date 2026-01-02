use brk_error::Result;
use brk_types::OHLCDollars;
use vecdb::Exit;

use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Timeindexes computed vecs
        self.timeindexes_to_price_close
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.close),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_high
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.high),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_low
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.low),
                    exit,
                )?;
                Ok(())
            })?;

        self.timeindexes_to_price_open
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_ohlc,
                    |(di, ohlc, ..)| (di, ohlc.open),
                    exit,
                )?;
                Ok(())
            })?;

        // Chainindexes computed vecs
        self.chainindexes_to_price_close
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.close),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_high
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.high),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_low
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.low),
                    exit,
                )?;
                Ok(())
            })?;

        self.chainindexes_to_price_open
            .compute(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_price_ohlc,
                    |(h, ohlc, ..)| (h, ohlc.open),
                    exit,
                )?;
                Ok(())
            })?;

        // Period OHLC aggregates
        self.weekindex_to_price_ohlc.compute_transform4(
            starting_indexes.weekindex,
            self.timeindexes_to_price_open.weekindex.unwrap_first(),
            self.timeindexes_to_price_high.weekindex.unwrap_max(),
            self.timeindexes_to_price_low.weekindex.unwrap_min(),
            self.timeindexes_to_price_close.weekindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.difficultyepoch_to_price_ohlc.compute_transform4(
            starting_indexes.difficultyepoch,
            self.chainindexes_to_price_open
                .difficultyepoch
                .unwrap_first(),
            self.chainindexes_to_price_high.difficultyepoch.unwrap_max(),
            self.chainindexes_to_price_low.difficultyepoch.unwrap_min(),
            self.chainindexes_to_price_close
                .difficultyepoch
                .unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.monthindex_to_price_ohlc.compute_transform4(
            starting_indexes.monthindex,
            self.timeindexes_to_price_open.monthindex.unwrap_first(),
            self.timeindexes_to_price_high.monthindex.unwrap_max(),
            self.timeindexes_to_price_low.monthindex.unwrap_min(),
            self.timeindexes_to_price_close.monthindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.quarterindex_to_price_ohlc.compute_transform4(
            starting_indexes.quarterindex,
            self.timeindexes_to_price_open.quarterindex.unwrap_first(),
            self.timeindexes_to_price_high.quarterindex.unwrap_max(),
            self.timeindexes_to_price_low.quarterindex.unwrap_min(),
            self.timeindexes_to_price_close.quarterindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.semesterindex_to_price_ohlc.compute_transform4(
            starting_indexes.semesterindex,
            self.timeindexes_to_price_open.semesterindex.unwrap_first(),
            self.timeindexes_to_price_high.semesterindex.unwrap_max(),
            self.timeindexes_to_price_low.semesterindex.unwrap_min(),
            self.timeindexes_to_price_close.semesterindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.yearindex_to_price_ohlc.compute_transform4(
            starting_indexes.yearindex,
            self.timeindexes_to_price_open.yearindex.unwrap_first(),
            self.timeindexes_to_price_high.yearindex.unwrap_max(),
            self.timeindexes_to_price_low.yearindex.unwrap_min(),
            self.timeindexes_to_price_close.yearindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
                        open,
                        high,
                        low,
                        close,
                    },
                )
            },
            exit,
        )?;

        self.decadeindex_to_price_ohlc.compute_transform4(
            starting_indexes.decadeindex,
            self.timeindexes_to_price_open.decadeindex.unwrap_first(),
            self.timeindexes_to_price_high.decadeindex.unwrap_max(),
            self.timeindexes_to_price_low.decadeindex.unwrap_min(),
            self.timeindexes_to_price_close.decadeindex.unwrap_last(),
            |(i, open, high, low, close, _)| {
                (
                    i,
                    OHLCDollars {
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
