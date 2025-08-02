#[derive(Default, Clone)]
pub struct GroupedByValue<T> {
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

impl<T> GroupedByValue<T> {
    pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
        vec![
            &mut self.up_to_1cent,
            &mut self.from_1c_to_10c,
            &mut self.from_10c_to_1d,
            &mut self.from_1d_to_10d,
            &mut self.from_10usd_to_100usd,
            &mut self.from_100usd_to_1_000usd,
            &mut self.from_1_000usd_to_10_000usd,
            &mut self.from_10_000usd_to_100_000usd,
            &mut self.from_100_000usd_to_1_000_000usd,
            &mut self.from_1_000_000usd_to_10_000_000usd,
            &mut self.from_10_000_000usd_to_100_000_000usd,
            &mut self.from_100_000_000usd_to_1_000_000_000usd,
            &mut self.from_1_000_000_000usd,
        ]
    }
}
