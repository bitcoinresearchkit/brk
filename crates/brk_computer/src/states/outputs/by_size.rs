#[derive(Default, Clone)]
pub struct OutputsBySize<T> {
    pub from_1sat_to_10sats: T,
    pub from_10sats_to_100sats: T,
    pub from_100sats_to_1_000sats: T,
    pub from_1_000sats_to_10_000sats: T,
    pub from_10_000sats_to_100_000sats: T,
    pub from_100_000sats_to_1_000_000sats: T,
    pub from_1_000_000sats_to_10_000_000sats: T,
    pub from_10_000_000sats_to_1btc: T,
    pub from_1btc_to_10btc: T,
    pub from_10btc_to_100btc: T,
    pub from_100btc_to_1_000btc: T,
    pub from_1_000btc_to_10_000btc: T,
    pub from_10_000btc_to_100_000btc: T,
    pub from_100_000btc: T,
}

impl<T> OutputsBySize<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        vec![
            &mut self.from_1sat_to_10sats,
            &mut self.from_10sats_to_100sats,
            &mut self.from_100sats_to_1_000sats,
            &mut self.from_1_000sats_to_10_000sats,
            &mut self.from_10_000sats_to_100_000sats,
            &mut self.from_100_000sats_to_1_000_000sats,
            &mut self.from_1_000_000sats_to_10_000_000sats,
            &mut self.from_10_000_000sats_to_1btc,
            &mut self.from_1btc_to_10btc,
            &mut self.from_10btc_to_100btc,
            &mut self.from_100btc_to_1_000btc,
            &mut self.from_1_000btc_to_10_000btc,
            &mut self.from_10_000btc_to_100_000btc,
            &mut self.from_100_000btc,
        ]
    }
}
