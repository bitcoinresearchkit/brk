use crate::structs::Amount;

#[derive(Debug, Default)]
pub struct InputState {
    count: f64,
    volume: Amount,
}

impl InputState {
    pub fn count(&self) -> f64 {
        self.count
    }

    pub fn volume(&self) -> Amount {
        self.volume
    }

    pub fn iterate(&mut self, count: f64, volume: Amount) {
        self.count += count;
        self.volume += volume;
    }
}
