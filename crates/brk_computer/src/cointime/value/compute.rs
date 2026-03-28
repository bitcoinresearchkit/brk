use brk_error::Result;
use brk_types::{Bitcoin, Dollars, Indexes, StoredF64};
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{distribution, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        prices: &prices::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let coinblocks_destroyed = &distribution.coinblocks_destroyed;
        let coindays_destroyed = &all_metrics.activity.coindays_destroyed;
        let circulating_supply = &all_metrics.supply.total.btc.height;

        self.destroyed
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &prices.spot.usd.height,
                    &coinblocks_destroyed.block,
                    exit,
                )?;
                Ok(())
            })?;

        self.created.compute(starting_indexes.height, exit, |vec| {
            vec.compute_multiply(
                starting_indexes.height,
                &prices.spot.usd.height,
                &activity.coinblocks_created.block,
                exit,
            )?;
            Ok(())
        })?;

        self.stored.compute(starting_indexes.height, exit, |vec| {
            vec.compute_multiply(
                starting_indexes.height,
                &prices.spot.usd.height,
                &activity.coinblocks_stored.block,
                exit,
            )?;
            Ok(())
        })?;

        // VOCDD: Value of Coin Days Destroyed = price × (CDD / circulating_supply)
        // Supply-adjusted to account for growing supply over time
        // This is a key input for Reserve Risk / HODL Bank calculation
        self.vocdd.compute(starting_indexes.height, exit, |vec| {
            vec.compute_transform3(
                starting_indexes.height,
                &prices.spot.usd.height,
                &coindays_destroyed.block,
                circulating_supply,
                |(i, price, cdd, supply, _): (_, Dollars, StoredF64, Bitcoin, _)| {
                    let supply_f64 = f64::from(supply);
                    if supply_f64 == 0.0 {
                        (i, StoredF64::from(0.0))
                    } else {
                        // VOCDD = price × (CDD / supply)
                        let vocdd = f64::from(price) * f64::from(cdd) / supply_f64;
                        (i, StoredF64::from(vocdd))
                    }
                },
                exit,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
