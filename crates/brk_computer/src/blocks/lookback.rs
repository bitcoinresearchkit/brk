use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Timestamp, Version};
use vecdb::{AnyVec, Cursor, Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableVec, Rw, StorageMode, VecIndex};

use crate::internal::WindowStarts;

use super::time;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub height_1h_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_24h_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 1d
    pub height_3d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_1w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 7d
    pub height_8d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_9d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_12d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_13d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_2w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 14d
    pub height_21d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_26d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_1m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 30d
    pub height_34d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_55d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_2m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 60d
    pub height_9w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 63d
    pub height_12w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 84d
    pub height_89d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_3m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 90d
    pub height_14w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 98d
    pub height_111d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_144d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_6m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 180d
    pub height_26w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 182d
    pub height_200d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_9m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 270d
    pub height_350d_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub height_12m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 360d
    pub height_1y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 365d
    pub height_14m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 420d
    pub height_2y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 730d
    pub height_26m_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 780d
    pub height_3y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1095d
    pub height_200w_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 1400d
    pub height_4y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1460d
    pub height_5y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1825d
    pub height_6y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 2190d
    pub height_8y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 2920d
    pub height_9y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 3285d
    pub height_10y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 3650d
    pub height_12y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 4380d
    pub height_14y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 5110d
    pub height_26y_ago: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 9490d
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_1h_ago: ImportableVec::forced_import(db, "height_1h_ago", version)?,
            height_24h_ago: ImportableVec::forced_import(db, "height_24h_ago", version)?,
            height_3d_ago: ImportableVec::forced_import(db, "height_3d_ago", version)?,
            height_1w_ago: ImportableVec::forced_import(db, "height_1w_ago", version)?,
            height_8d_ago: ImportableVec::forced_import(db, "height_8d_ago", version)?,
            height_9d_ago: ImportableVec::forced_import(db, "height_9d_ago", version)?,
            height_12d_ago: ImportableVec::forced_import(db, "height_12d_ago", version)?,
            height_13d_ago: ImportableVec::forced_import(db, "height_13d_ago", version)?,
            height_2w_ago: ImportableVec::forced_import(db, "height_2w_ago", version)?,
            height_21d_ago: ImportableVec::forced_import(db, "height_21d_ago", version)?,
            height_26d_ago: ImportableVec::forced_import(db, "height_26d_ago", version)?,
            height_1m_ago: ImportableVec::forced_import(db, "height_1m_ago", version)?,
            height_34d_ago: ImportableVec::forced_import(db, "height_34d_ago", version)?,
            height_55d_ago: ImportableVec::forced_import(db, "height_55d_ago", version)?,
            height_2m_ago: ImportableVec::forced_import(db, "height_2m_ago", version)?,
            height_9w_ago: ImportableVec::forced_import(db, "height_9w_ago", version)?,
            height_12w_ago: ImportableVec::forced_import(db, "height_12w_ago", version)?,
            height_89d_ago: ImportableVec::forced_import(db, "height_89d_ago", version)?,
            height_3m_ago: ImportableVec::forced_import(db, "height_3m_ago", version)?,
            height_14w_ago: ImportableVec::forced_import(db, "height_14w_ago", version)?,
            height_111d_ago: ImportableVec::forced_import(db, "height_111d_ago", version)?,
            height_144d_ago: ImportableVec::forced_import(db, "height_144d_ago", version)?,
            height_6m_ago: ImportableVec::forced_import(db, "height_6m_ago", version)?,
            height_26w_ago: ImportableVec::forced_import(db, "height_26w_ago", version)?,
            height_200d_ago: ImportableVec::forced_import(db, "height_200d_ago", version)?,
            height_9m_ago: ImportableVec::forced_import(db, "height_9m_ago", version)?,
            height_350d_ago: ImportableVec::forced_import(db, "height_350d_ago", version)?,
            height_12m_ago: ImportableVec::forced_import(db, "height_12m_ago", version)?,
            height_1y_ago: ImportableVec::forced_import(db, "height_1y_ago", version)?,
            height_14m_ago: ImportableVec::forced_import(db, "height_14m_ago", version)?,
            height_2y_ago: ImportableVec::forced_import(db, "height_2y_ago", version)?,
            height_26m_ago: ImportableVec::forced_import(db, "height_26m_ago", version)?,
            height_3y_ago: ImportableVec::forced_import(db, "height_3y_ago", version)?,
            height_200w_ago: ImportableVec::forced_import(db, "height_200w_ago", version)?,
            height_4y_ago: ImportableVec::forced_import(db, "height_4y_ago", version)?,
            height_5y_ago: ImportableVec::forced_import(db, "height_5y_ago", version)?,
            height_6y_ago: ImportableVec::forced_import(db, "height_6y_ago", version)?,
            height_8y_ago: ImportableVec::forced_import(db, "height_8y_ago", version)?,
            height_9y_ago: ImportableVec::forced_import(db, "height_9y_ago", version)?,
            height_10y_ago: ImportableVec::forced_import(db, "height_10y_ago", version)?,
            height_12y_ago: ImportableVec::forced_import(db, "height_12y_ago", version)?,
            height_14y_ago: ImportableVec::forced_import(db, "height_14y_ago", version)?,
            height_26y_ago: ImportableVec::forced_import(db, "height_26y_ago", version)?,
        })
    }

    pub fn window_starts(&self) -> WindowStarts<'_> {
        WindowStarts {
            _24h: &self.height_24h_ago,
            _1w: &self.height_1w_ago,
            _1m: &self.height_1m_ago,
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
            63 => &self.height_9w_ago,
            84 => &self.height_12w_ago,
            89 => &self.height_89d_ago,
            90 => &self.height_3m_ago,
            98 => &self.height_14w_ago,
            111 => &self.height_111d_ago,
            144 => &self.height_144d_ago,
            180 => &self.height_6m_ago,
            182 => &self.height_26w_ago,
            200 => &self.height_200d_ago,
            270 => &self.height_9m_ago,
            350 => &self.height_350d_ago,
            360 => &self.height_12m_ago,
            365 => &self.height_1y_ago,
            420 => &self.height_14m_ago,
            730 => &self.height_2y_ago,
            780 => &self.height_26m_ago,
            1095 => &self.height_3y_ago,
            1400 => &self.height_200w_ago,
            1460 => &self.height_4y_ago,
            1825 => &self.height_5y_ago,
            2190 => &self.height_6y_ago,
            2920 => &self.height_8y_ago,
            3285 => &self.height_9y_ago,
            3650 => &self.height_10y_ago,
            4380 => &self.height_12y_ago,
            5110 => &self.height_14y_ago,
            9490 => &self.height_26y_ago,
            _ => panic!("No start vec for {days} days"),
        }
    }

    pub(crate) fn compute(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_rolling_start_hours(time, starting_indexes, exit, 1, |s| {
            &mut s.height_1h_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1, |s| &mut s.height_24h_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 3, |s| &mut s.height_3d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 7, |s| &mut s.height_1w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 8, |s| &mut s.height_8d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 9, |s| &mut s.height_9d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 12, |s| &mut s.height_12d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 13, |s| &mut s.height_13d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 14, |s| &mut s.height_2w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 21, |s| &mut s.height_21d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 26, |s| &mut s.height_26d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 30, |s| &mut s.height_1m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 34, |s| &mut s.height_34d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 55, |s| &mut s.height_55d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 60, |s| &mut s.height_2m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 63, |s| &mut s.height_9w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 84, |s| &mut s.height_12w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 89, |s| &mut s.height_89d_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 90, |s| &mut s.height_3m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 98, |s| &mut s.height_14w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 111, |s| {
            &mut s.height_111d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 144, |s| {
            &mut s.height_144d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 180, |s| &mut s.height_6m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 182, |s| &mut s.height_26w_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 200, |s| {
            &mut s.height_200d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 270, |s| &mut s.height_9m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 350, |s| {
            &mut s.height_350d_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 360, |s| &mut s.height_12m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 365, |s| &mut s.height_1y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 420, |s| &mut s.height_14m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 730, |s| &mut s.height_2y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 780, |s| &mut s.height_26m_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 1095, |s| &mut s.height_3y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 1400, |s| {
            &mut s.height_200w_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 1460, |s| &mut s.height_4y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 1825, |s| &mut s.height_5y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 2190, |s| &mut s.height_6y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 2920, |s| &mut s.height_8y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 3285, |s| &mut s.height_9y_ago)?;
        self.compute_rolling_start(time, starting_indexes, exit, 3650, |s| {
            &mut s.height_10y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 4380, |s| {
            &mut s.height_12y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 5110, |s| {
            &mut s.height_14y_ago
        })?;
        self.compute_rolling_start(time, starting_indexes, exit, 9490, |s| {
            &mut s.height_26y_ago
        })?;

        Ok(())
    }

    fn compute_rolling_start<F>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        days: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(time, starting_indexes, exit, get_field, |t, prev_ts| {
            t.difference_in_days_between(prev_ts) >= days
        })
    }

    fn compute_rolling_start_hours<F>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        hours: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(time, starting_indexes, exit, get_field, |t, prev_ts| {
            t.difference_in_hours_between(prev_ts) >= hours
        })
    }

    fn compute_rolling_start_inner<F, D>(
        &mut self,
        time: &time::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        get_field: F,
        expired: D,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
        D: Fn(Timestamp, Timestamp) -> bool,
    {
        let field = get_field(self);
        let resume_from = field.len().min(starting_indexes.height.to_usize());
        let mut prev = if resume_from > 0 {
            field.collect_one_at(resume_from - 1).unwrap()
        } else {
            Height::ZERO
        };
        let mut cursor = Cursor::new(&time.timestamp_monotonic);
        cursor.advance(prev.to_usize());
        let mut prev_ts = cursor.next().unwrap();
        Ok(field.compute_transform(
            starting_indexes.height,
            &time.timestamp_monotonic,
            |(h, t, ..)| {
                while expired(t, prev_ts) {
                    prev.increment();
                    prev_ts = cursor.next().unwrap();
                    if prev > h {
                        unreachable!()
                    }
                }
                (h, prev)
            },
            exit,
        )?)
    }
}
