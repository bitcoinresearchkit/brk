use brk_traversable::Traversable;
use brk_types::{Height, StoredU32, StoredU64};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedFromHeightSumCum, ConstantVecs, RollingWindows, WindowStarts};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub block_count_target: ConstantVecs<StoredU64>,
    pub block_count: ComputedFromHeightSumCum<StoredU32, M>,
    pub block_count_sum: RollingWindows<StoredU32, M>,

    pub height_24h_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_3d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_1w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_8d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_9d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_12d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_13d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_2w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_21d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_26d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_1m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_34d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_55d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_2m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_89d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_111d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_144d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_3m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_6m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_200d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_350d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_1y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_2y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_200w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_3y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_4y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_5y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_6y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_8y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_10y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
}

impl Vecs {
    /// Get the standard 4 rolling window start heights (24h, 7d, 30d, 1y).
    pub fn window_starts(&self) -> WindowStarts<'_> {
        WindowStarts {
            _24h: &self.height_24h_ago,
            _7d: &self.height_1w_ago,
            _30d: &self.height_1m_ago,
            _1y: &self.height_1y_ago,
        }
    }

    pub fn start_vec(&self, days: usize) -> &EagerVec<PcoVec<Height, Height>> {
        match days {
            1 => &self.height_24h_ago,
            3 => &self.height_3d_ago,
            7 => &self.height_1w_ago,
            8 => &self.height_8d_ago,
            9 => &self.height_9d_ago,
            12 => &self.height_12d_ago,
            13 => &self.height_13d_ago,
            14 => &self.height_2w_ago,
            21 => &self.height_21d_ago,
            26 => &self.height_26d_ago,
            30 => &self.height_1m_ago,
            34 => &self.height_34d_ago,
            55 => &self.height_55d_ago,
            60 => &self.height_2m_ago,
            89 => &self.height_89d_ago,
            90 => &self.height_3m_ago,
            111 => &self.height_111d_ago,
            144 => &self.height_144d_ago,
            180 => &self.height_6m_ago,
            200 => &self.height_200d_ago,
            350 => &self.height_350d_ago,
            365 => &self.height_1y_ago,
            730 => &self.height_2y_ago,
            1095 => &self.height_3y_ago,
            1400 => &self.height_200w_ago,
            1460 => &self.height_4y_ago,
            1825 => &self.height_5y_ago,
            2190 => &self.height_6y_ago,
            2920 => &self.height_8y_ago,
            3650 => &self.height_10y_ago,
            _ => panic!("No start vec for {days} days"),
        }
    }
}
