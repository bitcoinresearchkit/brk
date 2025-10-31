use brk_error::Result;
use brk_types::{Bitcoin, CheckedSub, Close, Date, DateIndex, Dollars, Sats, StoredF32};
use vecdb::{
    AnyIterableVec, AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, StoredIndex,
    VecIterator, Version,
};

const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

pub trait ComputeDCAStackViaLen {
    fn compute_dca_stack_via_len(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>;

    fn compute_dca_stack_via_from(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()>;
}
impl ComputeDCAStackViaLen for EagerVec<DateIndex, Sats> {
    fn compute_dca_stack_via_len(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + closes.version(),
        )?;

        let mut other_iter = closes.iter();
        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes;
            let i_usize = i.to_usize();
            if prev.is_none() {
                if i_usize == 0 {
                    prev.replace(Sats::ZERO);
                } else {
                    prev.replace(self.into_iter().unwrap_get_inner_(i_usize - 1));
                }
            }

            let mut stack = Sats::ZERO;

            if price != Dollars::ZERO {
                stack = prev.unwrap() + Sats::from(Bitcoin::from(DCA_AMOUNT / price));

                if i_usize >= len {
                    let prev_price = *other_iter.unwrap_get_inner_(i_usize - len);
                    if prev_price != Dollars::ZERO {
                        stack = stack
                            .checked_sub(Sats::from(Bitcoin::from(DCA_AMOUNT / prev_price)))
                            .unwrap();
                    }
                }
            }

            prev.replace(stack);

            self.forced_push_at(i, stack, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    fn compute_dca_stack_via_from(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + closes.version(),
        )?;

        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes;
            let i_usize = i.to_usize();
            if prev.is_none() {
                if i_usize == 0 {
                    prev.replace(Sats::ZERO);
                } else {
                    prev.replace(self.into_iter().unwrap_get_inner_(i_usize - 1));
                }
            }

            let mut stack = Sats::ZERO;

            if price != Dollars::ZERO && i >= from {
                stack = prev.unwrap() + Sats::from(Bitcoin::from(DCA_AMOUNT / price));
            }

            prev.replace(stack);

            self.forced_push_at(i, stack, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }
}

pub trait ComputeDCAAveragePriceViaLen {
    fn compute_dca_avg_price_via_len(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>;

    fn compute_dca_avg_price_via_from(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()>;
}
impl ComputeDCAAveragePriceViaLen for EagerVec<DateIndex, Dollars> {
    fn compute_dca_avg_price_via_len(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ONE + self.inner_version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let first_price_date = DateIndex::try_from(Date::new(2010, 7, 12)).unwrap();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let mut avg_price = Dollars::from(f64::NAN);
            if i > first_price_date {
                avg_price = DCA_AMOUNT
                    * len
                        .min(i.to_usize() + 1)
                        .min(i.checked_sub(first_price_date).unwrap().to_usize() + 1)
                    / Bitcoin::from(stack);
            }
            self.forced_push_at(i, avg_price, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    fn compute_dca_avg_price_via_from(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let from_usize = from.to_usize();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let mut avg_price = Dollars::from(f64::NAN);
            if i >= from {
                avg_price = DCA_AMOUNT * (i.to_usize() + 1 - from_usize) / Bitcoin::from(stack);
            }
            self.forced_push_at(i, avg_price, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }
}

pub trait ComputeFromSats<I> {
    fn compute_from_sats(
        &mut self,
        max_from: I,
        sats: &impl AnyIterableVec<I, Sats>,
        exit: &Exit,
    ) -> Result<()>;
}
impl<I> ComputeFromSats<I> for EagerVec<I, Bitcoin>
where
    I: StoredIndex,
{
    fn compute_from_sats(
        &mut self,
        max_from: I,
        sats: &impl AnyIterableVec<I, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + sats.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        sats.iter_at(index).try_for_each(|(i, sats)| {
            let (i, v) = (i, Bitcoin::from(sats));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }
}

pub trait ComputeFromBitcoin<I> {
    fn compute_from_bitcoin(
        &mut self,
        max_from: I,
        bitcoin: &impl AnyIterableVec<I, Bitcoin>,
        price: &impl AnyIterableVec<I, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()>;
}
impl<I> ComputeFromBitcoin<I> for EagerVec<I, Dollars>
where
    I: StoredIndex,
{
    fn compute_from_bitcoin(
        &mut self,
        max_from: I,
        bitcoin: &impl AnyIterableVec<I, Bitcoin>,
        price: &impl AnyIterableVec<I, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + bitcoin.version(),
        )?;

        let mut price_iter = price.iter();
        let index = max_from.min(I::from(self.len()));
        bitcoin.iter_at(index).try_for_each(|(i, bitcoin)| {
            let dollars = price_iter.unwrap_get_inner(i);
            let (i, v) = (i, *dollars * bitcoin);
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }
}

pub trait ComputeDrawdown<I> {
    fn compute_drawdown(
        &mut self,
        max_from: I,
        close: &impl AnyIterableVec<I, Close<Dollars>>,
        ath: &impl AnyIterableVec<I, Dollars>,
        exit: &Exit,
    ) -> Result<()>;
}
impl<I> ComputeDrawdown<I> for EagerVec<I, StoredF32>
where
    I: StoredIndex,
{
    fn compute_drawdown(
        &mut self,
        max_from: I,
        close: &impl AnyIterableVec<I, Close<Dollars>>,
        ath: &impl AnyIterableVec<I, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + ath.version() + close.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut close_iter = close.iter();
        ath.iter_at(index).try_for_each(|(i, ath)| {
            if ath == Dollars::ZERO {
                self.forced_push_at(i, StoredF32::default(), exit)
            } else {
                let close = *close_iter.unwrap_get_inner(i);
                let drawdown = StoredF32::from((*ath - *close) / *ath * -100.0);
                self.forced_push_at(i, drawdown, exit)
            }
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }
}
