use std::borrow::Cow;

use bitcoin::Block;

pub trait BlockExtended {
    fn coinbase_tag(&self) -> Cow<'_, str>;
}

impl BlockExtended for Block {
    fn coinbase_tag(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(
            self.txdata
                .first()
                .and_then(|tx| tx.input.first())
                .unwrap()
                .script_sig
                .as_bytes(),
        )
    }
}
