#![allow(unused)]

#[derive(Default, Clone)]
pub struct Outputs<T> {
    pub all: T,
    // pub by_term: OutputsByTerm<T>,
    // pub by_up_to: OutputsByUpTo<T>,
    // pub by_from: OutputsByFrom<T>,
    // pub by_range: OutputsByRange<T>,
    // pub by_epoch: OutputsByEpoch<T>,
    // pub by_size: OutputsBySize<T>,
    // pub by_value: OutputsByValue<T>,
}

#[derive(Default)]
pub struct OutputsByTerm<T> {
    pub short: T,
    pub long: T,
}

#[derive(Default)]
pub struct OutputsByUpTo<T> {
    pub _1d: T,
    pub _1w: T,
    pub _1m: T,
    pub _2m: T,
    pub _3m: T,
    pub _4m: T,
    pub _5m: T,
    pub _6m: T,
    pub _1y: T,
    pub _2y: T,
    pub _3y: T,
    pub _5y: T,
    pub _7y: T,
    pub _10y: T,
    pub _15y: T,
}

#[derive(Default)]
pub struct OutputsByRange<T> {
    pub _1d_to_1w: T,
    pub _1w_to_1m: T,
    pub _1m_to_3m: T,
    pub _3m_to_6m: T,
    pub _6m_to_1y: T,
    pub _1y_to_2y: T,
    pub _2y_to_3y: T,
    pub _3y_to_5y: T,
    pub _5y_to_7y: T,
    pub _7y_to_10y: T,
    pub _10y_to_15y: T,
}

#[derive(Default)]
pub struct OutputsByFrom<T> {
    pub _1y: T,
    pub _2y: T,
    pub _4y: T,
    pub _10y: T,
    pub _15y: T,
}

#[derive(Default)]
pub struct OutputsByEpoch<T> {
    pub _1: T,
    pub _2: T,
    pub _3: T,
    pub _4: T,
    pub _5: T,
}

#[derive(Default)]
pub struct OutputsBySize<T> {
    pub from_1_to_10: T,
    pub from_10_to_100: T,
    pub from_100_to_1_000: T,
    pub from_1_000_to_10_000: T,
    pub from_10000_to_100_000: T,
    pub from_100_000_to_1_000_000: T,
    pub from_1_000_000_to_10_000_000: T,
    pub from_10_000_000_to_1btc: T,
    pub from_1btc_to_10btc: T,
    pub from_10btc_to_100btc: T,
    pub from_100btc_to_1_000btc: T,
    pub from_1_000btc_to_10_000btc: T,
    pub from_10_000btc_to_100_000btc: T,
    pub from_100_000btc: T,
}

#[derive(Default)]
pub struct OutputsByValue<T> {
    pub up_to_1cent: T,
    pub from_1c_to_10c: T,
    pub from_10c_to_1d: T,
    pub from_1d_to_10d: T,
    pub from_10usd_to_100usd: T,
    pub from_100usd_to_1_000usd: T,
    pub from_1_000usd_to_10_000usd: T,
    pub from_10_000usd_to_100_000usd: T,
    pub from_100_000usd_to_1_000_000usd: T,
    pub from_1_000_000usd_to_10_000_000usd: T,
    pub from_10_000_000usd_to_100_000_000usd: T,
    pub from_100_000_000usd_to_1_000_000_000usd: T,
    pub from_1_000_000_000usd: T,
    // ...
}
