use crate::structs::Amount;

#[derive(Debug, Default)]
pub struct OutputState {
    count: f64,
    volume: Amount,
}

impl OutputState {
    // pub fn count(&self) -> f64 {
    //     self.count
    // }

    // pub fn volume(&self) -> Amount {
    //     self.volume
    // }

    pub fn iterate(&mut self, count: f64, volume: Amount) {
        self.count += count;
        self.volume += volume;
    }
}
