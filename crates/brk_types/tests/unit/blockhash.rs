use std::fmt;

use bitcoin::{BlockHash as BitcoinBlockHash, hashes::Hash};
#[cfg(feature = "storage")]
use vecdb::Formattable;

use super::BlockHash;

struct Previous(BlockHash);

impl fmt::Display for Previous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&BitcoinBlockHash::from(&self.0).to_string())
    }
}

#[test]
fn formatting_preserves_bitcoin_order_and_wire_bytes() {
    for seed in 0..=u8::MAX {
        let native =
            BitcoinBlockHash::from_byte_array(std::array::from_fn(|i| seed.wrapping_add(i as u8)));
        let hash = BlockHash::from(native);
        let expected = native.to_string();
        assert_eq!(hash.to_string(), expected);
        assert_eq!(format!("{hash:>80}"), format!("{:>80}", Previous(hash)));
        assert_eq!(format!("{hash:.8}"), format!("{:.8}", Previous(hash)));
        let json = serde_json::to_string(&hash).unwrap();
        assert_eq!(json, format!("\"{expected}\""));
        assert_eq!(serde_json::from_str::<BlockHash>(&json).unwrap(), hash);
        #[cfg(feature = "storage")]
        {
            let mut bytes = vec![b'!'];
            hash.fmt_json(&mut bytes);
            assert_eq!(&bytes[1..], json.as_bytes());
        }
    }
}
