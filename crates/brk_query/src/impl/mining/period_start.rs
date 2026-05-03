use brk_types::{Height, TimePeriod};

use crate::Query;

impl Query {
    /// First block height inside `period` looking back from the tip; genesis (0) for `All`.
    pub(super) fn start_height(&self, period: TimePeriod) -> Height {
        self.computer()
            .blocks
            .lookback
            .start_height(period, self.height())
            .unwrap_or_default()
    }
}
