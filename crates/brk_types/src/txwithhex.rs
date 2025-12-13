use crate::Transaction;

/// A transaction with its raw hex representation
#[derive(Debug, Clone)]
pub struct TxWithHex {
    tx: Transaction,
    hex: String,
}

impl TxWithHex {
    pub fn new(tx: Transaction, hex: String) -> Self {
        Self { tx, hex }
    }

    pub fn tx(&self) -> &Transaction {
        &self.tx
    }

    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn into_parts(self) -> (Transaction, String) {
        (self.tx, self.hex)
    }
}
