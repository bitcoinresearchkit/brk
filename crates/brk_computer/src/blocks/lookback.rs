use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Timestamp, Version};
use vecdb::{AnyVec, CachedVec, Cursor, Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableVec, Rw, StorageMode, VecIndex};

use crate::{
    indexes,
    internal::{CachedWindowStarts, Windows, WindowStarts},
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub cached_window_starts: CachedWindowStarts,
    pub _1h: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _24h: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 1d
    pub _3d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _1w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 7d
    pub _8d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _9d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _12d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _13d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _2w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 14d
    pub _21d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _26d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _1m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 30d
    pub _34d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _55d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _2m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 60d
    pub _9w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 63d
    pub _12w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 84d
    pub _89d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _3m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 90d
    pub _14w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 98d
    pub _111d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _144d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _6m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 180d
    pub _26w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 182d
    pub _200d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _9m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 270d
    pub _350d: M::Stored<EagerVec<PcoVec<Height, Height>>>,
    pub _12m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 360d
    pub _1y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 365d
    pub _14m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 420d
    pub _2y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 730d
    pub _26m: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 780d
    pub _3y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1095d
    pub _200w: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 1400d
    pub _4y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1460d
    pub _5y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 1825d
    pub _6y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 2190d
    pub _8y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 2920d
    pub _9y: M::Stored<EagerVec<PcoVec<Height, Height>>>,  // 3285d
    pub _10y: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 3650d
    pub _12y: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 4380d
    pub _14y: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 5110d
    pub _26y: M::Stored<EagerVec<PcoVec<Height, Height>>>, // 9490d
}

impl Vecs {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let _1h = ImportableVec::forced_import(db, "height_1h_ago", version)?;
        let _24h = ImportableVec::forced_import(db, "height_24h_ago", version)?;
        let _3d = ImportableVec::forced_import(db, "height_3d_ago", version)?;
        let _1w = ImportableVec::forced_import(db, "height_1w_ago", version)?;
        let _8d = ImportableVec::forced_import(db, "height_8d_ago", version)?;
        let _9d = ImportableVec::forced_import(db, "height_9d_ago", version)?;
        let _12d = ImportableVec::forced_import(db, "height_12d_ago", version)?;
        let _13d = ImportableVec::forced_import(db, "height_13d_ago", version)?;
        let _2w = ImportableVec::forced_import(db, "height_2w_ago", version)?;
        let _21d = ImportableVec::forced_import(db, "height_21d_ago", version)?;
        let _26d = ImportableVec::forced_import(db, "height_26d_ago", version)?;
        let _1m = ImportableVec::forced_import(db, "height_1m_ago", version)?;
        let _34d = ImportableVec::forced_import(db, "height_34d_ago", version)?;
        let _55d = ImportableVec::forced_import(db, "height_55d_ago", version)?;
        let _2m = ImportableVec::forced_import(db, "height_2m_ago", version)?;
        let _9w = ImportableVec::forced_import(db, "height_9w_ago", version)?;
        let _12w = ImportableVec::forced_import(db, "height_12w_ago", version)?;
        let _89d = ImportableVec::forced_import(db, "height_89d_ago", version)?;
        let _3m = ImportableVec::forced_import(db, "height_3m_ago", version)?;
        let _14w = ImportableVec::forced_import(db, "height_14w_ago", version)?;
        let _111d = ImportableVec::forced_import(db, "height_111d_ago", version)?;
        let _144d = ImportableVec::forced_import(db, "height_144d_ago", version)?;
        let _6m = ImportableVec::forced_import(db, "height_6m_ago", version)?;
        let _26w = ImportableVec::forced_import(db, "height_26w_ago", version)?;
        let _200d = ImportableVec::forced_import(db, "height_200d_ago", version)?;
        let _9m = ImportableVec::forced_import(db, "height_9m_ago", version)?;
        let _350d = ImportableVec::forced_import(db, "height_350d_ago", version)?;
        let _12m = ImportableVec::forced_import(db, "height_12m_ago", version)?;
        let _1y = ImportableVec::forced_import(db, "height_1y_ago", version)?;
        let _14m = ImportableVec::forced_import(db, "height_14m_ago", version)?;
        let _2y = ImportableVec::forced_import(db, "height_2y_ago", version)?;
        let _26m = ImportableVec::forced_import(db, "height_26m_ago", version)?;
        let _3y = ImportableVec::forced_import(db, "height_3y_ago", version)?;
        let _200w = ImportableVec::forced_import(db, "height_200w_ago", version)?;
        let _4y = ImportableVec::forced_import(db, "height_4y_ago", version)?;
        let _5y = ImportableVec::forced_import(db, "height_5y_ago", version)?;
        let _6y = ImportableVec::forced_import(db, "height_6y_ago", version)?;
        let _8y = ImportableVec::forced_import(db, "height_8y_ago", version)?;
        let _9y = ImportableVec::forced_import(db, "height_9y_ago", version)?;
        let _10y = ImportableVec::forced_import(db, "height_10y_ago", version)?;
        let _12y = ImportableVec::forced_import(db, "height_12y_ago", version)?;
        let _14y = ImportableVec::forced_import(db, "height_14y_ago", version)?;
        let _26y = ImportableVec::forced_import(db, "height_26y_ago", version)?;

        let cached_window_starts = CachedWindowStarts(Windows {
            _24h: CachedVec::new(&_24h),
            _1w: CachedVec::new(&_1w),
            _1m: CachedVec::new(&_1m),
            _1y: CachedVec::new(&_1y),
        });

        Ok(Self {
            cached_window_starts,
            _1h, _24h, _3d, _1w, _8d, _9d, _12d, _13d, _2w, _21d, _26d,
            _1m, _34d, _55d, _2m, _9w, _12w, _89d, _3m, _14w, _111d, _144d,
            _6m, _26w, _200d, _9m, _350d, _12m, _1y, _14m, _2y, _26m, _3y,
            _200w, _4y, _5y, _6y, _8y, _9y, _10y, _12y, _14y, _26y,
        })
    }

    pub fn window_starts(&self) -> WindowStarts<'_> {
        WindowStarts {
            _24h: &self._24h,
            _1w: &self._1w,
            _1m: &self._1m,
            _1y: &self._1y,
        }
    }


    pub fn start_vec(&self, days: usize) -> &EagerVec<PcoVec<Height, Height>> {
        match days {
            1 => &self._24h,
            3 => &self._3d,
            7 => &self._1w,
            8 => &self._8d,
            9 => &self._9d,
            12 => &self._12d,
            13 => &self._13d,
            14 => &self._2w,
            21 => &self._21d,
            26 => &self._26d,
            30 => &self._1m,
            34 => &self._34d,
            55 => &self._55d,
            60 => &self._2m,
            63 => &self._9w,
            84 => &self._12w,
            89 => &self._89d,
            90 => &self._3m,
            98 => &self._14w,
            111 => &self._111d,
            144 => &self._144d,
            180 => &self._6m,
            182 => &self._26w,
            200 => &self._200d,
            270 => &self._9m,
            350 => &self._350d,
            360 => &self._12m,
            365 => &self._1y,
            420 => &self._14m,
            730 => &self._2y,
            780 => &self._26m,
            1095 => &self._3y,
            1400 => &self._200w,
            1460 => &self._4y,
            1825 => &self._5y,
            2190 => &self._6y,
            2920 => &self._8y,
            3285 => &self._9y,
            3650 => &self._10y,
            4380 => &self._12y,
            5110 => &self._14y,
            9490 => &self._26y,
            _ => panic!("No start vec for {days} days"),
        }
    }

    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_rolling_start_hours(indexes, starting_indexes, exit, 1, |s| {
            &mut s._1h
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 1, |s| &mut s._24h)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 3, |s| &mut s._3d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 7, |s| &mut s._1w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 8, |s| &mut s._8d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 9, |s| &mut s._9d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 12, |s| &mut s._12d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 13, |s| &mut s._13d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 14, |s| &mut s._2w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 21, |s| &mut s._21d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 26, |s| &mut s._26d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 30, |s| &mut s._1m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 34, |s| &mut s._34d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 55, |s| &mut s._55d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 60, |s| &mut s._2m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 63, |s| &mut s._9w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 84, |s| &mut s._12w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 89, |s| &mut s._89d)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 90, |s| &mut s._3m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 98, |s| &mut s._14w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 111, |s| {
            &mut s._111d
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 144, |s| {
            &mut s._144d
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 180, |s| &mut s._6m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 182, |s| &mut s._26w)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 200, |s| {
            &mut s._200d
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 270, |s| &mut s._9m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 350, |s| {
            &mut s._350d
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 360, |s| &mut s._12m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 365, |s| &mut s._1y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 420, |s| &mut s._14m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 730, |s| &mut s._2y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 780, |s| &mut s._26m)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 1095, |s| &mut s._3y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 1400, |s| {
            &mut s._200w
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 1460, |s| &mut s._4y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 1825, |s| &mut s._5y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 2190, |s| &mut s._6y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 2920, |s| &mut s._8y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 3285, |s| &mut s._9y)?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 3650, |s| {
            &mut s._10y
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 4380, |s| {
            &mut s._12y
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 5110, |s| {
            &mut s._14y
        })?;
        self.compute_rolling_start(indexes, starting_indexes, exit, 9490, |s| {
            &mut s._26y
        })?;

        Ok(())
    }

    fn compute_rolling_start<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        days: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(indexes, starting_indexes, exit, get_field, |t, prev_ts| {
            t.difference_in_days_between(prev_ts) >= days
        })
    }

    fn compute_rolling_start_hours<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        hours: usize,
        get_field: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self) -> &mut EagerVec<PcoVec<Height, Height>>,
    {
        self.compute_rolling_start_inner(indexes, starting_indexes, exit, get_field, |t, prev_ts| {
            t.difference_in_hours_between(prev_ts) >= hours
        })
    }

    fn compute_rolling_start_inner<F, D>(
        &mut self,
        indexes: &indexes::Vecs,
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
        let mut cursor = Cursor::new(&indexes.timestamp.monotonic);
        cursor.advance(prev.to_usize());
        let mut prev_ts = cursor.next().unwrap();
        Ok(field.compute_transform(
            starting_indexes.height,
            &indexes.timestamp.monotonic,
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
