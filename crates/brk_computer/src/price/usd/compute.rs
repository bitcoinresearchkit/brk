use brk_error::Result;
use brk_types::{Close, Dollars, High, Low, OHLCDollars, Open};
use vecdb::Exit;

use super::super::cents;
use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        cents: &cents::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Open: first-value aggregation
        self.split.open.height.compute_transform(
            starting_indexes.height,
            &cents.split.height.open,
            |(h, open, ..)| (h, Open::new(Dollars::from(*open))),
            exit,
        )?;
        self.split.open.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &cents.split.dateindex.open,
                |(di, open, ..)| (di, Open::new(Dollars::from(*open))),
                exit,
            )?;
            Ok(())
        })?;

        // High: max-value aggregation
        self.split.high.height.compute_transform(
            starting_indexes.height,
            &cents.split.height.high,
            |(h, high, ..)| (h, High::new(Dollars::from(*high))),
            exit,
        )?;
        self.split.high.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &cents.split.dateindex.high,
                |(di, high, ..)| (di, High::new(Dollars::from(*high))),
                exit,
            )?;
            Ok(())
        })?;

        // Low: min-value aggregation
        self.split.low.height.compute_transform(
            starting_indexes.height,
            &cents.split.height.low,
            |(h, low, ..)| (h, Low::new(Dollars::from(*low))),
            exit,
        )?;
        self.split.low.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &cents.split.dateindex.low,
                |(di, low, ..)| (di, Low::new(Dollars::from(*low))),
                exit,
            )?;
            Ok(())
        })?;

        // Close: last-value aggregation
        self.split.close.height.compute_transform(
            starting_indexes.height,
            &cents.split.height.close,
            |(h, close, ..)| (h, Close::new(Dollars::from(*close))),
            exit,
        )?;
        self.split.close.compute_rest(starting_indexes, exit, |v| {
            v.compute_transform(
                starting_indexes.dateindex,
                &cents.split.dateindex.close,
                |(di, close, ..)| (di, Close::new(Dollars::from(*close))),
                exit,
            )?;
            Ok(())
        })?;

        // Period OHLC aggregates - time based
        self.ohlc.dateindex.compute_transform4(
            starting_indexes.dateindex,
            &self.split.open.dateindex,
            &self.split.high.dateindex,
            &self.split.low.dateindex,
            &self.split.close.dateindex,
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

        self.ohlc.week.compute_transform4(
            starting_indexes.weekindex,
            &*self.split.open.weekindex,
            &*self.split.high.weekindex,
            &*self.split.low.weekindex,
            &*self.split.close.weekindex,
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

        self.ohlc.month.compute_transform4(
            starting_indexes.monthindex,
            &*self.split.open.monthindex,
            &*self.split.high.monthindex,
            &*self.split.low.monthindex,
            &*self.split.close.monthindex,
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

        self.ohlc.quarter.compute_transform4(
            starting_indexes.quarterindex,
            &*self.split.open.quarterindex,
            &*self.split.high.quarterindex,
            &*self.split.low.quarterindex,
            &*self.split.close.quarterindex,
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

        self.ohlc.semester.compute_transform4(
            starting_indexes.semesterindex,
            &*self.split.open.semesterindex,
            &*self.split.high.semesterindex,
            &*self.split.low.semesterindex,
            &*self.split.close.semesterindex,
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

        self.ohlc.year.compute_transform4(
            starting_indexes.yearindex,
            &*self.split.open.yearindex,
            &*self.split.high.yearindex,
            &*self.split.low.yearindex,
            &*self.split.close.yearindex,
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

        self.ohlc.decade.compute_transform4(
            starting_indexes.decadeindex,
            &*self.split.open.decadeindex,
            &*self.split.high.decadeindex,
            &*self.split.low.decadeindex,
            &*self.split.close.decadeindex,
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

        // Period OHLC aggregates - chain based
        self.ohlc.height.compute_transform4(
            starting_indexes.height,
            &self.split.open.height,
            &self.split.high.height,
            &self.split.low.height,
            &self.split.close.height,
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

        self.ohlc.difficultyepoch.compute_transform4(
            starting_indexes.difficultyepoch,
            &*self.split.open.difficultyepoch,
            &*self.split.high.difficultyepoch,
            &*self.split.low.difficultyepoch,
            &*self.split.close.difficultyepoch,
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
