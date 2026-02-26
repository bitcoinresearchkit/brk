use brk_types::{Day1, Dollars, Month1, StoredF32, Week1, Year1};
use vecdb::{ReadableOptionVec, VecIndex};

use crate::{market::returns::Vecs as ReturnsVecs, prices};

pub(super) fn collect_returns(tf: &str, returns: &ReturnsVecs) -> Vec<f32> {
    let data: Vec<StoredF32> = match tf {
        "1d" => returns.price_returns._1d.day1.collect_or_default(),
        "1w" => returns.price_returns._1w.week1.collect_or_default(),
        "1m" => returns.price_returns._1m.month1.collect_or_default(),
        "1y" => returns.price_returns._1y.year1.collect_or_default(),
        _ => unreachable!(),
    };
    data.into_iter().map(|v| *v).collect()
}

pub(super) fn collect_closes(tf: &str, prices: &prices::Vecs) -> Vec<Dollars> {
    match tf {
        "1d" => prices.usd.close.day1.collect_or_default(),
        "1w" => prices.usd.close.week1.collect_or_default(),
        "1m" => prices.usd.close.month1.collect_or_default(),
        "1y" => prices.usd.close.year1.collect_or_default(),
        _ => unreachable!(),
    }
}

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
