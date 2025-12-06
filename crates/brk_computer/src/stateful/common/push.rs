//! Push and flush methods for Vecs.
//!
//! This module contains methods for:
//! - `truncate_push`: Push state values to height-indexed vectors
//! - `compute_then_truncate_push_unrealized_states`: Compute and push unrealized states
//! - `safe_flush_stateful_vecs`: Safely flush all stateful vectors

use brk_error::Result;
use brk_types::{DateIndex, Dollars, Height, StoredU64};
use vecdb::{AnyStoredVec, Exit, GenericStoredVec};

use crate::{stateful::Flushable, states::CohortState, utils::OptionExt};

use super::Vecs;

impl Vecs {
    pub fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.height_to_supply
            .truncate_push(height, state.supply.value)?;

        self.height_to_utxo_count
            .truncate_push(height, StoredU64::from(state.supply.utxo_count))?;

        self.height_to_sent.truncate_push(height, state.sent)?;

        self.height_to_satblocks_destroyed
            .truncate_push(height, state.satblocks_destroyed)?;

        self.height_to_satdays_destroyed
            .truncate_push(height, state.satdays_destroyed)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            let realized = state.realized.as_ref().unwrap_or_else(|| {
                dbg!((&state.realized, &state.supply));
                panic!();
            });

            height_to_realized_cap.truncate_push(height, realized.cap)?;

            self.height_to_realized_profit
                .um()
                .truncate_push(height, realized.profit)?;
            self.height_to_realized_loss
                .um()
                .truncate_push(height, realized.loss)?;
            self.height_to_value_created
                .um()
                .truncate_push(height, realized.value_created)?;
            self.height_to_value_destroyed
                .um()
                .truncate_push(height, realized.value_destroyed)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .truncate_push(height, realized.adj_value_created)?;
                self.height_to_adjusted_value_destroyed
                    .um()
                    .truncate_push(height, realized.adj_value_destroyed)?;
            }
        }
        Ok(())
    }

    pub fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        state: &CohortState,
    ) -> Result<()> {
        if let Some(height_price) = height_price {
            self.height_to_min_price_paid.um().truncate_push(
                height,
                state
                    .price_to_amount_first_key_value()
                    .map(|(&dollars, _)| dollars)
                    .unwrap_or(Dollars::NAN),
            )?;
            self.height_to_max_price_paid.um().truncate_push(
                height,
                state
                    .price_to_amount_last_key_value()
                    .map(|(&dollars, _)| dollars)
                    .unwrap_or(Dollars::NAN),
            )?;

            let (height_unrealized_state, date_unrealized_state) =
                state.compute_unrealized_states(height_price, date_price.unwrap());

            self.height_to_supply_in_profit
                .um()
                .truncate_push(height, height_unrealized_state.supply_in_profit)?;
            self.height_to_supply_in_loss
                .um()
                .truncate_push(height, height_unrealized_state.supply_in_loss)?;
            self.height_to_unrealized_profit
                .um()
                .truncate_push(height, height_unrealized_state.unrealized_profit)?;
            self.height_to_unrealized_loss
                .um()
                .truncate_push(height, height_unrealized_state.unrealized_loss)?;

            if let Some(date_unrealized_state) = date_unrealized_state {
                let dateindex = dateindex.unwrap();

                self.dateindex_to_supply_in_profit
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_profit)?;
                self.dateindex_to_supply_in_loss
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.supply_in_loss)?;
                self.dateindex_to_unrealized_profit
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.unrealized_profit)?;
                self.dateindex_to_unrealized_loss
                    .um()
                    .truncate_push(dateindex, date_unrealized_state.unrealized_loss)?;
            }

            // Compute and push price percentiles
            if let Some(price_percentiles) = self.price_percentiles.as_mut() {
                let percentile_prices = state.compute_percentile_prices();
                price_percentiles.truncate_push(height, &percentile_prices)?;
            }
        }

        Ok(())
    }

    pub fn safe_flush_stateful_vecs(
        &mut self,
        height: Height,
        exit: &Exit,
        state: &mut CohortState,
    ) -> Result<()> {
        self.height_to_supply.safe_write(exit)?;
        self.height_to_utxo_count.safe_write(exit)?;
        self.height_to_sent.safe_write(exit)?;
        self.height_to_satdays_destroyed.safe_write(exit)?;
        self.height_to_satblocks_destroyed.safe_write(exit)?;

        if let Some(height_to_realized_cap) = self.height_to_realized_cap.as_mut() {
            height_to_realized_cap.safe_write(exit)?;
            self.height_to_realized_profit.um().safe_write(exit)?;
            self.height_to_realized_loss.um().safe_write(exit)?;
            self.height_to_value_created.um().safe_write(exit)?;
            self.height_to_value_destroyed.um().safe_write(exit)?;
            self.height_to_supply_in_profit.um().safe_write(exit)?;
            self.height_to_supply_in_loss.um().safe_write(exit)?;
            self.height_to_unrealized_profit.um().safe_write(exit)?;
            self.height_to_unrealized_loss.um().safe_write(exit)?;
            self.dateindex_to_supply_in_profit.um().safe_write(exit)?;
            self.dateindex_to_supply_in_loss.um().safe_write(exit)?;
            self.dateindex_to_unrealized_profit.um().safe_write(exit)?;
            self.dateindex_to_unrealized_loss.um().safe_write(exit)?;
            self.height_to_min_price_paid.um().safe_write(exit)?;
            self.height_to_max_price_paid.um().safe_write(exit)?;

            if self.height_to_adjusted_value_created.is_some() {
                self.height_to_adjusted_value_created
                    .um()
                    .safe_write(exit)?;
                self.height_to_adjusted_value_destroyed
                    .um()
                    .safe_write(exit)?;
            }

            // Uses Flushable trait - Option<T> impl handles None case
            self.price_percentiles.safe_write(exit)?;
        }

        state.commit(height)?;

        Ok(())
    }
}
