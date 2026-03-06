use std::cmp::Ordering;

use brk_types::{Cents, CentsSats, CentsSquaredSats, Sats};

/// Trait for realized state operations, implemented by Minimal, Core, and Full variants.
pub trait RealizedOps: Default + Clone + Send + Sync + 'static {
    fn cap(&self) -> Cents;
    fn profit(&self) -> Cents;
    fn loss(&self) -> Cents;
    fn value_created(&self) -> Cents {
        Cents::ZERO
    }
    fn value_destroyed(&self) -> Cents {
        Cents::ZERO
    }
    fn set_cap_raw(&mut self, cap_raw: CentsSats);
    fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats);
    fn reset_single_iteration_values(&mut self);
    fn increment(&mut self, price: Cents, sats: Sats);
    fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats);
    fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats);
    fn receive(&mut self, price: Cents, sats: Sats) {
        self.increment(price, sats);
    }
    fn send(
        &mut self,
        sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    );
}

/// Minimal realized state: only cap, profit, loss.
/// Used by MinimalCohortMetrics cohorts (amount_range, type_, address — ~135 separate cohorts).
#[derive(Debug, Default, Clone)]
pub struct MinimalRealizedState {
    cap_raw: u128,
    profit_raw: u128,
    loss_raw: u128,
}

impl RealizedOps for MinimalRealizedState {
    #[inline]
    fn cap(&self) -> Cents {
        if self.cap_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.cap_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn profit(&self) -> Cents {
        if self.profit_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn loss(&self) -> Cents {
        if self.loss_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.cap_raw = cap_raw.inner();
    }

    #[inline]
    fn set_investor_cap_raw(&mut self, _investor_cap_raw: CentsSquaredSats) {}

    #[inline]
    fn reset_single_iteration_values(&mut self) {
        self.profit_raw = 0;
        self.loss_raw = 0;
    }

    #[inline]
    fn increment(&mut self, price: Cents, sats: Sats) {
        if sats.is_zero() {
            return;
        }
        let price_sats = CentsSats::from_price_sats(price, sats);
        self.cap_raw += price_sats.as_u128();
    }

    #[inline]
    fn increment_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.cap_raw += price_sats.as_u128();
    }

    #[inline]
    fn decrement_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.cap_raw -= price_sats.as_u128();
    }

    #[inline]
    fn send(
        &mut self,
        _sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        _ath_ps: CentsSats,
        _prev_investor_cap: CentsSquaredSats,
    ) {
        match current_ps.cmp(&prev_ps) {
            Ordering::Greater => {
                self.profit_raw += (current_ps - prev_ps).as_u128();
            }
            Ordering::Less => {
                self.loss_raw += (prev_ps - current_ps).as_u128();
            }
            Ordering::Equal => {}
        }
        self.cap_raw -= prev_ps.as_u128();
    }
}

/// Core realized state: cap, profit, loss + value_created/destroyed for SOPR.
/// Used by CoreCohortMetrics cohorts (epoch, class, max_age, min_age — ~59 separate cohorts).
#[derive(Debug, Default, Clone)]
pub struct CoreRealizedState {
    minimal: MinimalRealizedState,
    value_created_raw: u128,
    value_destroyed_raw: u128,
}

impl RealizedOps for CoreRealizedState {
    #[inline]
    fn cap(&self) -> Cents {
        self.minimal.cap()
    }

    #[inline]
    fn profit(&self) -> Cents {
        self.minimal.profit()
    }

    #[inline]
    fn loss(&self) -> Cents {
        self.minimal.loss()
    }

    #[inline]
    fn value_created(&self) -> Cents {
        if self.value_created_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn value_destroyed(&self) -> Cents {
        if self.value_destroyed_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.minimal.set_cap_raw(cap_raw);
    }

    #[inline]
    fn set_investor_cap_raw(&mut self, _investor_cap_raw: CentsSquaredSats) {}

    #[inline]
    fn reset_single_iteration_values(&mut self) {
        self.minimal.reset_single_iteration_values();
        self.value_created_raw = 0;
        self.value_destroyed_raw = 0;
    }

    #[inline]
    fn increment(&mut self, price: Cents, sats: Sats) {
        self.minimal.increment(price, sats);
    }

    #[inline]
    fn increment_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.minimal.increment_snapshot(price_sats, _investor_cap);
    }

    #[inline]
    fn decrement_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.minimal.decrement_snapshot(price_sats, _investor_cap);
    }

    #[inline]
    fn send(
        &mut self,
        sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    ) {
        self.minimal
            .send(sats, current_ps, prev_ps, ath_ps, prev_investor_cap);
        self.value_created_raw += current_ps.as_u128();
        self.value_destroyed_raw += prev_ps.as_u128();
    }
}

impl CoreRealizedState {
    #[inline(always)]
    pub(super) fn cap_raw_u128(&self) -> u128 {
        self.minimal.cap_raw
    }
}

/// Full realized state (~160 bytes).
/// Used by BasicCohortMetrics cohorts (age_range — 21 separate cohorts).
#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    core: CoreRealizedState,
    /// Raw investor cap: Σ(price² × sats)
    investor_cap_raw: CentsSquaredSats,
    /// sell_price × sats for profit cases
    profit_value_created_raw: u128,
    /// cost_basis × sats for profit cases
    profit_value_destroyed_raw: u128,
    /// sell_price × sats for loss cases
    loss_value_created_raw: u128,
    /// cost_basis × sats for loss cases (= capitulation_flow)
    loss_value_destroyed_raw: u128,
    /// Raw realized peak regret: Σ((peak - sell_price) × sats)
    peak_regret_raw: u128,
    /// Sats sent in profit
    sent_in_profit: Sats,
    /// Sats sent in loss
    sent_in_loss: Sats,
}

impl RealizedOps for RealizedState {
    #[inline]
    fn cap(&self) -> Cents {
        self.core.cap()
    }

    #[inline]
    fn profit(&self) -> Cents {
        self.core.profit()
    }

    #[inline]
    fn loss(&self) -> Cents {
        self.core.loss()
    }

    #[inline]
    fn value_created(&self) -> Cents {
        let raw = self.profit_value_created_raw + self.loss_value_created_raw;
        if raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn value_destroyed(&self) -> Cents {
        let raw = self.profit_value_destroyed_raw + self.loss_value_destroyed_raw;
        if raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.core.set_cap_raw(cap_raw);
    }

    #[inline]
    fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats) {
        self.investor_cap_raw = investor_cap_raw;
    }

    #[inline]
    fn reset_single_iteration_values(&mut self) {
        self.core.reset_single_iteration_values();
        self.profit_value_created_raw = 0;
        self.profit_value_destroyed_raw = 0;
        self.loss_value_created_raw = 0;
        self.loss_value_destroyed_raw = 0;
        self.peak_regret_raw = 0;
        self.sent_in_profit = Sats::ZERO;
        self.sent_in_loss = Sats::ZERO;
    }

    #[inline]
    fn increment(&mut self, price: Cents, sats: Sats) {
        self.core.increment(price, sats);
        if sats.is_not_zero() {
            self.investor_cap_raw += CentsSats::from_price_sats(price, sats).to_investor_cap(price);
        }
    }

    #[inline]
    fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.core.increment_snapshot(price_sats, investor_cap);
        self.investor_cap_raw += investor_cap;
    }

    #[inline]
    fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.core.decrement_snapshot(price_sats, investor_cap);
        self.investor_cap_raw -= investor_cap;
    }

    #[inline]
    fn send(
        &mut self,
        sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    ) {
        // Delegate cap/profit/loss + value_created/destroyed to core
        self.core
            .send(sats, current_ps, prev_ps, ath_ps, prev_investor_cap);

        // Per-component value flow tracking
        let current = current_ps.as_u128();
        let prev = prev_ps.as_u128();
        match current_ps.cmp(&prev_ps) {
            Ordering::Greater => {
                self.profit_value_created_raw += current;
                self.profit_value_destroyed_raw += prev;
                self.sent_in_profit += sats;
            }
            Ordering::Less => {
                self.loss_value_created_raw += current;
                self.loss_value_destroyed_raw += prev;
                self.sent_in_loss += sats;
            }
            Ordering::Equal => {
                self.profit_value_created_raw += current;
                self.profit_value_destroyed_raw += prev;
                self.sent_in_profit += sats;
            }
        }

        self.peak_regret_raw += (ath_ps - current_ps).as_u128();
        self.investor_cap_raw -= prev_investor_cap;
    }
}

impl RealizedState {
    /// Get investor price as CentsUnsigned.
    /// investor_price = Σ(price² × sats) / Σ(price × sats)
    #[inline]
    pub(crate) fn investor_price(&self) -> Cents {
        let cap_raw = self.core.cap_raw_u128();
        if cap_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.investor_cap_raw / cap_raw) as u64)
    }

    /// Get raw realized cap for aggregation.
    #[inline]
    pub(crate) fn cap_raw(&self) -> CentsSats {
        CentsSats::new(self.core.cap_raw_u128())
    }

    /// Get raw investor cap for aggregation.
    #[inline]
    pub(crate) fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.investor_cap_raw
    }

    /// Get profit value created as CentsUnsigned (sell_price × sats for profit cases).
    #[inline]
    pub(crate) fn profit_value_created(&self) -> Cents {
        if self.profit_value_created_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value destroyed as CentsUnsigned (cost_basis × sats for profit cases).
    #[inline]
    pub(crate) fn profit_value_destroyed(&self) -> Cents {
        if self.profit_value_destroyed_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value created as CentsUnsigned (sell_price × sats for loss cases).
    #[inline]
    pub(crate) fn loss_value_created(&self) -> Cents {
        if self.loss_value_created_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value destroyed as CentsUnsigned (cost_basis × sats for loss cases).
    #[inline]
    pub(crate) fn loss_value_destroyed(&self) -> Cents {
        if self.loss_value_destroyed_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get realized peak regret as CentsUnsigned.
    #[inline]
    pub(crate) fn peak_regret(&self) -> Cents {
        if self.peak_regret_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.peak_regret_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get sats sent in profit.
    #[inline]
    pub(crate) fn sent_in_profit(&self) -> Sats {
        self.sent_in_profit
    }

    /// Get sats sent in loss.
    #[inline]
    pub(crate) fn sent_in_loss(&self) -> Sats {
        self.sent_in_loss
    }
}
