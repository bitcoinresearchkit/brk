use super::Amount;

#[derive(Default, Debug)]
pub struct SentData {
    pub volume: Amount,
    pub count: u32,
}

impl SentData {
    pub fn send(&mut self, amount: Amount) {
        self.volume += amount;
        self.count += 1;
    }
}
