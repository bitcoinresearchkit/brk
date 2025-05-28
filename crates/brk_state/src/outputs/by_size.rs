use super::OutputFilter;

#[derive(Default, Clone)]
pub struct OutputsBySize<T> {
    pub _0sat: T,
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

impl<T> From<OutputsBySize<T>> for OutputsBySize<(OutputFilter, T)> {
    fn from(value: OutputsBySize<T>) -> Self {
        #[allow(clippy::inconsistent_digit_grouping)]
        Self {
            _0sat: (
                // OutputFilter::Zero,
                OutputFilter::Size,
                value._0sat,
            ),
            from_1sat_to_10sats: (
                // OutputFilter::Size(Sats::new(1)..Sats::new(10)),
                OutputFilter::Size,
                value.from_1sat_to_10sats,
            ),
            from_10sats_to_100sats: (
                // OutputFilter::Size(Sats::new(10)..Sats::new(100)),
                OutputFilter::Size,
                value.from_10sats_to_100sats,
            ),
            from_100sats_to_1_000sats: (
                // OutputFilter::Size(Sats::new(100)..Sats::new(1_000)),
                OutputFilter::Size,
                value.from_100sats_to_1_000sats,
            ),
            from_1_000sats_to_10_000sats: (
                // OutputFilter::Size(Sats::new(1_000)..Sats::new(10_000)),
                OutputFilter::Size,
                value.from_1_000sats_to_10_000sats,
            ),
            from_10_000sats_to_100_000sats: (
                // OutputFilter::Size(Sats::new(10_000)..Sats::new(100_000)),
                OutputFilter::Size,
                value.from_10_000sats_to_100_000sats,
            ),
            from_100_000sats_to_1_000_000sats: (
                // OutputFilter::Size(Sats::new(100_000)..Sats::new(1_000_000)),
                OutputFilter::Size,
                value.from_100_000sats_to_1_000_000sats,
            ),
            from_1_000_000sats_to_10_000_000sats: (
                // OutputFilter::Size(Sats::new(1_000_000)..Sats::new(10_000_000)),
                OutputFilter::Size,
                value.from_1_000_000sats_to_10_000_000sats,
            ),
            from_10_000_000sats_to_1btc: (
                // OutputFilter::Size(Sats::new(10_000_000)..Sats::new(1_00_000_000)),
                OutputFilter::Size,
                value.from_10_000_000sats_to_1btc,
            ),
            from_1btc_to_10btc: (
                // OutputFilter::Size(Sats::new(1_00_000_000)..Sats::new(10_00_000_000)),
                OutputFilter::Size,
                value.from_1btc_to_10btc,
            ),
            from_10btc_to_100btc: (
                // OutputFilter::Size(Sats::new(10_00_000_000)..Sats::new(100_00_000_000)),
                OutputFilter::Size,
                value.from_10btc_to_100btc,
            ),
            from_100btc_to_1_000btc: (
                // OutputFilter::Size(Sats::new(100_00_000_000)..Sats::new(1_000_00_000_000)),
                OutputFilter::Size,
                value.from_100btc_to_1_000btc,
            ),
            from_1_000btc_to_10_000btc: (
                // OutputFilter::Size(Sats::new(1_000_00_000_000)..Sats::new(10_000_00_000_000)),
                OutputFilter::Size,
                value.from_1_000btc_to_10_000btc,
            ),
            from_10_000btc_to_100_000btc: (
                // OutputFilter::Size(Sats::new(10_000_00_000_000)..Sats::new(100_000_00_000_000)),
                OutputFilter::Size,
                value.from_10_000btc_to_100_000btc,
            ),
            from_100_000btc: (
                // OutputFilter::Size(Sats::new(100_000_00_000_000)..Sats::MAX),
                OutputFilter::Size,
                value.from_100_000btc,
            ),
        }
    }
}

impl<T> OutputsBySize<T> {
    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn get_mut(&mut self, group: usize) -> &mut T {
        if group == 0 {
            &mut self._0sat
        } else if group == 1 {
            &mut self.from_1sat_to_10sats
        } else if group == 10 {
            &mut self.from_10sats_to_100sats
        } else if group == 100 {
            &mut self.from_100sats_to_1_000sats
        } else if group == 1_000 {
            &mut self.from_1_000sats_to_10_000sats
        } else if group == 10_000 {
            &mut self.from_10_000sats_to_100_000sats
        } else if group == 100_000 {
            &mut self.from_100_000sats_to_1_000_000sats
        } else if group == 1_000_000 {
            &mut self.from_1_000_000sats_to_10_000_000sats
        } else if group == 10_000_000 {
            &mut self.from_10_000_000sats_to_1btc
        } else if group == 1_00_000_000 {
            &mut self.from_1btc_to_10btc
        } else if group == 10_00_000_000 {
            &mut self.from_10btc_to_100btc
        } else if group == 100_00_000_000 {
            &mut self.from_100btc_to_1_000btc
        } else if group == 1_000_00_000_000 {
            &mut self.from_1_000btc_to_10_000btc
        } else if group == 10_000_00_000_000 {
            &mut self.from_10_000btc_to_100_000btc
        } else {
            &mut self.from_100_000btc
        }
    }

    pub fn as_mut_vec(&mut self) -> [&mut T; 15] {
        [
            &mut self._0sat,
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

impl<T> OutputsBySize<(OutputFilter, T)> {
    pub fn vecs(&self) -> [&T; 15] {
        [
            &self._0sat.1,
            &self.from_1sat_to_10sats.1,
            &self.from_10sats_to_100sats.1,
            &self.from_100sats_to_1_000sats.1,
            &self.from_1_000sats_to_10_000sats.1,
            &self.from_10_000sats_to_100_000sats.1,
            &self.from_100_000sats_to_1_000_000sats.1,
            &self.from_1_000_000sats_to_10_000_000sats.1,
            &self.from_10_000_000sats_to_1btc.1,
            &self.from_1btc_to_10btc.1,
            &self.from_10btc_to_100btc.1,
            &self.from_100btc_to_1_000btc.1,
            &self.from_1_000btc_to_10_000btc.1,
            &self.from_10_000btc_to_100_000btc.1,
            &self.from_100_000btc.1,
        ]
    }
}
