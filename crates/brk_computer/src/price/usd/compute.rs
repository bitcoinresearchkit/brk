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
            &*self.timeindexes_to_price_open.weekindex,
            &*self.timeindexes_to_price_high.weekindex,
            &*self.timeindexes_to_price_low.weekindex,
            &*self.timeindexes_to_price_close.weekindex,
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
            &*self.chainindexes_to_price_open.difficultyepoch,
            &*self.chainindexes_to_price_high.difficultyepoch,
            &*self.chainindexes_to_price_low.difficultyepoch,
            &*self.chainindexes_to_price_close.difficultyepoch,
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
            &*self.timeindexes_to_price_open.monthindex,
            &*self.timeindexes_to_price_high.monthindex,
            &*self.timeindexes_to_price_low.monthindex,
            &*self.timeindexes_to_price_close.monthindex,
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
            &*self.timeindexes_to_price_open.quarterindex,
            &*self.timeindexes_to_price_high.quarterindex,
            &*self.timeindexes_to_price_low.quarterindex,
            &*self.timeindexes_to_price_close.quarterindex,
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
            &*self.timeindexes_to_price_open.semesterindex,
            &*self.timeindexes_to_price_high.semesterindex,
            &*self.timeindexes_to_price_low.semesterindex,
            &*self.timeindexes_to_price_close.semesterindex,
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
            &*self.timeindexes_to_price_open.yearindex,
            &*self.timeindexes_to_price_high.yearindex,
            &*self.timeindexes_to_price_low.yearindex,
            &*self.timeindexes_to_price_close.yearindex,
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
            &*self.timeindexes_to_price_open.decadeindex,
            &*self.timeindexes_to_price_high.decadeindex,
            &*self.timeindexes_to_price_low.decadeindex,
            &*self.timeindexes_to_price_close.decadeindex,
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
