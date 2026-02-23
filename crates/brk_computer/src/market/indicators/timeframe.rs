use brk_types::{Day1, Dollars, Month1, StoredF32, Week1, Year1};
use vecdb::{ReadableVec, VecIndex};

use crate::{market::returns::Vecs as ReturnsVecs, prices};

/// Returns period-level returns data
pub(super) fn collect_returns(tf: &str, returns: &ReturnsVecs) -> Vec<f32> {
    match tf {
        "1d" => {
            let data: Vec<StoredF32> = returns.price_returns._1d.day1.collect();
            data.into_iter().map(|v| *v).collect()
        }
        "1w" => {
            let data: Vec<StoredF32> = returns.price_returns._1w.week1.collect();
            data.into_iter().map(|v| *v).collect()
        }
        "1m" => {
            let data: Vec<StoredF32> = returns.price_returns._1m.month1.collect();
            data.into_iter().map(|v| *v).collect()
        }
        "1y" => {
            let data: Vec<StoredF32> = returns.price_returns._1y.year1.collect();
            data.into_iter().map(|v| *v).collect()
        }
        _ => unreachable!(),
    }
}

/// Returns period-level close prices
pub(super) fn collect_closes(tf: &str, prices: &prices::Vecs) -> Vec<Dollars> {
    match tf {
        "1d" => prices.usd.split.close.day1.collect(),
        "1w" => prices.usd.split.close.week1.collect(),
        "1m" => prices.usd.split.close.month1.collect(),
        "1y" => prices.usd.split.close.year1.collect(),
        _ => unreachable!(),
    }
}

/// Maps a Day1 to a period-level index for the given timeframe
#[inline]
pub(super) fn date_to_period(tf: &str, di: Day1) -> usize {
    match tf {
        "1d" => di.to_usize(),
        "1w" => Week1::from(di).to_usize(),
        "1m" => Month1::from(di).to_usize(),
        "1y" => Year1::from(Month1::from(di)).to_usize(),
        _ => unreachable!(),
    }
}
