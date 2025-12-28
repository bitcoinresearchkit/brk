use brk_error::Result;
use brk_types::{Bitcoin, CheckedSub, Close, Date, DateIndex, Dollars, Sats, StoredF32};
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, IterableVec, PcoVec, VecIndex, Version,
};

const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

pub trait ComputeDCAStackViaLen {
    fn compute_dca_stack_via_len(
        &mut self,
        max_from: DateIndex,
        closes: &impl IterableVec<DateIndex, Close<Dollars>>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>;

    fn compute_dca_stack_via_from(
        &mut self,
        max_from: DateIndex,
        closes: &impl IterableVec<DateIndex, Close<Dollars>>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()>;
}

impl ComputeDCAStackViaLen for EagerVec<PcoVec<DateIndex, Sats>> {
    fn compute_dca_stack_via_len(
        &mut self,
        max_from: DateIndex,
        closes: &impl IterableVec<DateIndex, Close<Dollars>>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + closes.version(),
        )?;

        let index = max_from.to_usize().min(self.len());

        // Initialize prev before the loop to avoid checking on every iteration
        let mut prev = if index == 0 {
            Sats::ZERO
        } else {
            self.read_at_unwrap_once(index - 1)
        };

        let mut lookback = closes.create_lookback(index, len, 0);

        closes
            .iter()
            .enumerate()
            .skip(index)
            .try_for_each(|(i, closes)| {
                let price = *closes;
                let i_usize = i.to_usize();

                let mut stack = Sats::ZERO;

                if price != Dollars::ZERO {
                    stack = prev + Sats::from(Bitcoin::from(DCA_AMOUNT / price));

                    let prev_price =
                        *lookback.get_and_push(i_usize, Close::new(price), Close::default());
                    if prev_price != Dollars::ZERO {
                        stack = stack
                            .checked_sub(Sats::from(Bitcoin::from(DCA_AMOUNT / prev_price)))
                            .unwrap();
                    }
                }

                prev = stack;

                self.truncate_push_at(i, stack)
            })?;

        self.safe_write(exit)?;

        Ok(())
    }

    fn compute_dca_stack_via_from(
        &mut self,
        max_from: DateIndex,
        closes: &impl IterableVec<DateIndex, Close<Dollars>>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + closes.version(),
        )?;

        let from = from.to_usize();
        let index = max_from.min(DateIndex::from(self.len()));

        // Initialize prev before the loop to avoid checking on every iteration
        let mut prev = if index.to_usize() == 0 {
            Sats::ZERO
        } else {
            self.read_at_unwrap_once(index.to_usize() - 1)
        };

        closes
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, closes)| {
                let price = *closes;

                let mut stack = Sats::ZERO;

                if price != Dollars::ZERO && i >= from {
                    stack = prev + Sats::from(Bitcoin::from(DCA_AMOUNT / price));
                }

                prev = stack;

                self.truncate_push_at(i, stack)
            })?;

        self.safe_write(exit)?;

        Ok(())
    }
}

pub trait ComputeDCAAveragePriceViaLen {
    fn compute_dca_avg_price_via_len(
        &mut self,
        max_from: DateIndex,
        stacks: &impl IterableVec<DateIndex, Sats>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>;

    fn compute_dca_avg_price_via_from(
        &mut self,
        max_from: DateIndex,
        stacks: &impl IterableVec<DateIndex, Sats>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()>;
}

impl ComputeDCAAveragePriceViaLen for EagerVec<PcoVec<DateIndex, Dollars>> {
    fn compute_dca_avg_price_via_len(
        &mut self,
        max_from: DateIndex,
        stacks: &impl IterableVec<DateIndex, Sats>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ONE + self.inner_version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let first_price_date = DateIndex::try_from(Date::new(2010, 7, 12))
            .unwrap()
            .to_usize();

        stacks
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, stack)| {
                let mut avg_price = Dollars::from(f64::NAN);
                if i > first_price_date {
                    avg_price = DCA_AMOUNT
                        * len
                            .min(i.to_usize() + 1)
                            .min(i.checked_sub(first_price_date).unwrap().to_usize() + 1)
                        / Bitcoin::from(stack);
                }
                self.truncate_push_at(i, avg_price)
            })?;

        self.safe_write(exit)?;

        Ok(())
    }

    fn compute_dca_avg_price_via_from(
        &mut self,
        max_from: DateIndex,
        stacks: &impl IterableVec<DateIndex, Sats>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset(
            Version::ZERO + self.inner_version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let from = from.to_usize();

        stacks
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, stack)| {
                let mut avg_price = Dollars::from(f64::NAN);
                if i >= from {
                    avg_price = DCA_AMOUNT * (i.to_usize() + 1 - from) / Bitcoin::from(stack);
                }
                self.truncate_push_at(i, avg_price)
            })?;

        self.safe_write(exit)?;

        Ok(())
    }
}

pub trait ComputeFromBitcoin<I> {
    fn compute_from_bitcoin(
        &mut self,
        max_from: I,
        bitcoin: &impl IterableVec<I, Bitcoin>,
        price: &impl IterableVec<I, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()>;
}

impl<I> ComputeFromBitcoin<I> for EagerVec<PcoVec<I, Dollars>>
where
    I: VecIndex,
{
    fn compute_from_bitcoin(
        &mut self,
        max_from: I,
        bitcoin: &impl IterableVec<I, Bitcoin>,
        price: &impl IterableVec<I, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_transform2(
            max_from,
            bitcoin,
            price,
            |(i, bitcoin, price, _)| (i, *price * bitcoin),
            exit,
        )?;
        Ok(())
    }
}

pub trait ComputeDrawdown<I> {
    fn compute_drawdown(
        &mut self,
        max_from: I,
        close: &impl IterableVec<I, Close<Dollars>>,
        ath: &impl IterableVec<I, Dollars>,
        exit: &Exit,
    ) -> Result<()>;
}

impl<I> ComputeDrawdown<I> for EagerVec<PcoVec<I, StoredF32>>
where
    I: VecIndex,
{
    fn compute_drawdown(
        &mut self,
        max_from: I,
        close: &impl IterableVec<I, Close<Dollars>>,
        ath: &impl IterableVec<I, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_transform2(
            max_from,
            ath,
            close,
            |(i, ath, close, _)| {
                if ath == Dollars::ZERO {
                    (i, StoredF32::default())
                } else {
                    let drawdown = StoredF32::from((*ath - **close) / *ath * -100.0);
                    (i, drawdown)
                }
            },
            exit,
        )?;
        Ok(())
    }
}
