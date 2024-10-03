use std::ops::AddAssign;

use allocative::Allocative;
use color_eyre::eyre::eyre;

#[derive(Debug, Default, Allocative)]
pub struct UTXOState {
    pub count: f64,
}

impl UTXOState {
    pub fn increment(&mut self, utxo_count: f64) {
        self.count += utxo_count;
    }

    pub fn decrement(&mut self, utxo_count: f64) -> color_eyre::Result<()> {
        if self.count < utxo_count {
            dbg!(self.count, utxo_count);

            return Err(eyre!("self.count smaller than utxo_count"));
        }

        self.count -= utxo_count;

        Ok(())
    }
}

impl AddAssign for UTXOState {
    fn add_assign(&mut self, rhs: Self) {
        self.count += rhs.count;
    }
}
