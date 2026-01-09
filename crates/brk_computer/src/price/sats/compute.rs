use brk_error::Result;
use brk_types::{Close, High, Low, OHLCSats, Open, Sats};
use vecdb::Exit;

use super::super::usd;
use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        usd: &usd::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Open: first-value aggregation (1 BTC / price)
        self.split.open.height.compute_transform(
            starting_indexes.height,
            &usd.split.open.height,
            |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
            exit,
        )?;
        self.split
            .open
            .compute_rest(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.split.open.dateindex,
                    |(i, open, ..)| (i, Open::new(Sats::ONE_BTC / *open)),
                    exit,
                )?;
                Ok(())
            })?;

        // High: max-value aggregation (sats high = 1 BTC / usd low)
        self.split.high.height.compute_transform(
            starting_indexes.height,
            &usd.split.low.height,
            |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
            exit,
        )?;
        self.split
            .high
            .compute_rest(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.split.low.dateindex,
                    |(i, low, ..)| (i, High::new(Sats::ONE_BTC / *low)),
                    exit,
                )?;
                Ok(())
            })?;

        // Low: min-value aggregation (sats low = 1 BTC / usd high)
        self.split.low.height.compute_transform(
            starting_indexes.height,
            &usd.split.high.height,
            |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
            exit,
        )?;
        self.split.low.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &usd.split.high.dateindex,
                |(i, high, ..)| (i, Low::new(Sats::ONE_BTC / *high)),
                exit,
            )?;
            Ok(())
        })?;

        // Close: last-value aggregation
        self.split.close.height.compute_transform(
            starting_indexes.height,
            &usd.split.close.height,
            |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
            exit,
        )?;
        self.split
            .close
            .compute_rest(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    &usd.split.close.dateindex,
                    |(i, close, ..)| (i, Close::new(Sats::ONE_BTC / *close)),
                    exit,
                )?;
                Ok(())
            })?;

        // Height OHLC in sats
        self.ohlc.height.compute_transform4(
            starting_indexes.height,
            &self.split.open.height,
            &self.split.high.height,
            &self.split.low.height,
            &self.split.close.height,
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
        self.ohlc.dateindex.compute_transform4(
            starting_indexes.dateindex,
            &self.split.open.dateindex,
            &self.split.high.dateindex,
            &self.split.low.dateindex,
            &self.split.close.dateindex,
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
        self.ohlc.week.compute_transform4(
            starting_indexes.weekindex,
            &*self.split.open.weekindex,
            &*self.split.high.weekindex,
            &*self.split.low.weekindex,
            &*self.split.close.weekindex,
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

        self.ohlc.difficultyepoch.compute_transform4(
            starting_indexes.difficultyepoch,
            &*self.split.open.difficultyepoch,
            &*self.split.high.difficultyepoch,
            &*self.split.low.difficultyepoch,
            &*self.split.close.difficultyepoch,
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

        self.ohlc.month.compute_transform4(
            starting_indexes.monthindex,
            &*self.split.open.monthindex,
            &*self.split.high.monthindex,
            &*self.split.low.monthindex,
            &*self.split.close.monthindex,
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

        self.ohlc.quarter.compute_transform4(
            starting_indexes.quarterindex,
            &*self.split.open.quarterindex,
            &*self.split.high.quarterindex,
            &*self.split.low.quarterindex,
            &*self.split.close.quarterindex,
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

        self.ohlc.semester.compute_transform4(
            starting_indexes.semesterindex,
            &*self.split.open.semesterindex,
            &*self.split.high.semesterindex,
            &*self.split.low.semesterindex,
            &*self.split.close.semesterindex,
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

        self.ohlc.year.compute_transform4(
            starting_indexes.yearindex,
            &*self.split.open.yearindex,
            &*self.split.high.yearindex,
            &*self.split.low.yearindex,
            &*self.split.close.yearindex,
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

        self.ohlc.decade.compute_transform4(
            starting_indexes.decadeindex,
            &*self.split.open.decadeindex,
            &*self.split.high.decadeindex,
            &*self.split.low.decadeindex,
            &*self.split.close.decadeindex,
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
