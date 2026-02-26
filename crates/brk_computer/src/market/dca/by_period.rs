use brk_traversable::Traversable;

/// DCA period identifiers with their day counts
pub const DCA_PERIOD_DAYS: ByDcaPeriod<u32> = ByDcaPeriod {
    _1w: 7,
    _1m: 30,
    _3m: 3 * 30,
    _6m: 6 * 30,
    _1y: 365,
    _2y: 2 * 365,
    _3y: 3 * 365,
    _4y: 4 * 365,
    _5y: 5 * 365,
    _6y: 6 * 365,
    _8y: 8 * 365,
    _10y: 10 * 365,
};

/// DCA period names
pub const DCA_PERIOD_NAMES: ByDcaPeriod<&'static str> = ByDcaPeriod {
    _1w: "1w",
    _1m: "1m",
    _3m: "3m",
    _6m: "6m",
    _1y: "1y",
    _2y: "2y",
    _3y: "3y",
    _4y: "4y",
    _5y: "5y",
    _6y: "6y",
    _8y: "8y",
    _10y: "10y",
};

/// DCA CAGR period days (only periods >= 2y)
pub const DCA_CAGR_DAYS: ByDcaCagr<u32> = ByDcaCagr {
    _2y: 2 * 365,
    _3y: 3 * 365,
    _4y: 4 * 365,
    _5y: 5 * 365,
    _6y: 6 * 365,
    _8y: 8 * 365,
    _10y: 10 * 365,
};

/// DCA CAGR period names
pub const DCA_CAGR_NAMES: ByDcaCagr<&'static str> = ByDcaCagr {
    _2y: "2y",
    _3y: "3y",
    _4y: "4y",
    _5y: "5y",
    _6y: "6y",
    _8y: "8y",
    _10y: "10y",
};

/// Generic wrapper for DCA period-based data
#[derive(Clone, Default, Traversable)]
pub struct ByDcaPeriod<T> {
    pub _1w: T,
    pub _1m: T,
    pub _3m: T,
    pub _6m: T,
    pub _1y: T,
    pub _2y: T,
    pub _3y: T,
    pub _4y: T,
    pub _5y: T,
    pub _6y: T,
    pub _8y: T,
    pub _10y: T,
}

impl<T> ByDcaPeriod<T> {
    pub(crate) fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str, u32) -> Result<T, E>,
    {
        let n = DCA_PERIOD_NAMES;
        let d = DCA_PERIOD_DAYS;
        Ok(Self {
            _1w: create(n._1w, d._1w)?,
            _1m: create(n._1m, d._1m)?,
            _3m: create(n._3m, d._3m)?,
            _6m: create(n._6m, d._6m)?,
            _1y: create(n._1y, d._1y)?,
            _2y: create(n._2y, d._2y)?,
            _3y: create(n._3y, d._3y)?,
            _4y: create(n._4y, d._4y)?,
            _5y: create(n._5y, d._5y)?,
            _6y: create(n._6y, d._6y)?,
            _8y: create(n._8y, d._8y)?,
            _10y: create(n._10y, d._10y)?,
        })
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._1w,
            &mut self._1m,
            &mut self._3m,
            &mut self._6m,
            &mut self._1y,
            &mut self._2y,
            &mut self._3y,
            &mut self._4y,
            &mut self._5y,
            &mut self._6y,
            &mut self._8y,
            &mut self._10y,
        ]
        .into_iter()
    }

    pub(crate) fn iter_with_days(&self) -> impl Iterator<Item = (&T, u32)> {
        let d = DCA_PERIOD_DAYS;
        [
            (&self._1w, d._1w),
            (&self._1m, d._1m),
            (&self._3m, d._3m),
            (&self._6m, d._6m),
            (&self._1y, d._1y),
            (&self._2y, d._2y),
            (&self._3y, d._3y),
            (&self._4y, d._4y),
            (&self._5y, d._5y),
            (&self._6y, d._6y),
            (&self._8y, d._8y),
            (&self._10y, d._10y),
        ]
        .into_iter()
    }

    pub(crate) fn iter_mut_with_days(&mut self) -> impl Iterator<Item = (&mut T, u32)> {
        let d = DCA_PERIOD_DAYS;
        [
            (&mut self._1w, d._1w),
            (&mut self._1m, d._1m),
            (&mut self._3m, d._3m),
            (&mut self._6m, d._6m),
            (&mut self._1y, d._1y),
            (&mut self._2y, d._2y),
            (&mut self._3y, d._3y),
            (&mut self._4y, d._4y),
            (&mut self._5y, d._5y),
            (&mut self._6y, d._6y),
            (&mut self._8y, d._8y),
            (&mut self._10y, d._10y),
        ]
        .into_iter()
    }

    pub(crate) fn zip_mut_with_days<'a, U>(
        &'a mut self,
        other: &'a ByDcaPeriod<U>,
    ) -> impl Iterator<Item = (&'a mut T, &'a U, u32)> {
        let d = DCA_PERIOD_DAYS;
        [
            (&mut self._1w, &other._1w, d._1w),
            (&mut self._1m, &other._1m, d._1m),
            (&mut self._3m, &other._3m, d._3m),
            (&mut self._6m, &other._6m, d._6m),
            (&mut self._1y, &other._1y, d._1y),
            (&mut self._2y, &other._2y, d._2y),
            (&mut self._3y, &other._3y, d._3y),
            (&mut self._4y, &other._4y, d._4y),
            (&mut self._5y, &other._5y, d._5y),
            (&mut self._6y, &other._6y, d._6y),
            (&mut self._8y, &other._8y, d._8y),
            (&mut self._10y, &other._10y, d._10y),
        ]
        .into_iter()
    }

}

/// Generic wrapper for DCA CAGR data (periods >= 2 years)
#[derive(Clone, Default, Traversable)]
pub struct ByDcaCagr<T> {
    pub _2y: T,
    pub _3y: T,
    pub _4y: T,
    pub _5y: T,
    pub _6y: T,
    pub _8y: T,
    pub _10y: T,
}

impl<T> ByDcaCagr<T> {
    pub(crate) fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str, u32) -> Result<T, E>,
    {
        let n = DCA_CAGR_NAMES;
        let d = DCA_CAGR_DAYS;
        Ok(Self {
            _2y: create(n._2y, d._2y)?,
            _3y: create(n._3y, d._3y)?,
            _4y: create(n._4y, d._4y)?,
            _5y: create(n._5y, d._5y)?,
            _6y: create(n._6y, d._6y)?,
            _8y: create(n._8y, d._8y)?,
            _10y: create(n._10y, d._10y)?,
        })
    }

    /// Zip with the matching subset of a ByDcaPeriod
    pub(crate) fn zip_mut_with_period<'a, U>(
        &'a mut self,
        period: &'a ByDcaPeriod<U>,
    ) -> impl Iterator<Item = (&'a mut T, &'a U, u32)> {
        let d = DCA_CAGR_DAYS;
        [
            (&mut self._2y, &period._2y, d._2y),
            (&mut self._3y, &period._3y, d._3y),
            (&mut self._4y, &period._4y, d._4y),
            (&mut self._5y, &period._5y, d._5y),
            (&mut self._6y, &period._6y, d._6y),
            (&mut self._8y, &period._8y, d._8y),
            (&mut self._10y, &period._10y, d._10y),
        ]
        .into_iter()
    }
}
