use bitcoin::Block;

pub trait BlockExtended {
    fn coinbase_tag(&self) -> String;
}

impl BlockExtended for Block {
    fn coinbase_tag(&self) -> String {
        let Some(input) = self.txdata.first().and_then(|tx| tx.input.first()) else {
            return String::new();
        };
        let bytes = input.script_sig.as_bytes();
        String::from_utf8_lossy(bytes)
            .chars()
            .filter(|&c| c != '\u{FFFD}' && (c >= ' ' || c == '\n' || c == '\r' || c == '\t'))
            .take(1_024)
            .collect()
    }
}
