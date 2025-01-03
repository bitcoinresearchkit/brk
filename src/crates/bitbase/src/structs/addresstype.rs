use biter::bitcoin::ScriptBuf;
use color_eyre::eyre::eyre;
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, Clone, Copy)]
pub enum Addresstype {
    P2PK,
    P2PKH,
}

impl TryFrom<&ScriptBuf> for Addresstype {
    type Error = color_eyre::Report;
    fn try_from(value: &ScriptBuf) -> Result<Self, Self::Error> {
        if value.is_p2pk() {
            Ok(Self::P2PK)
        } else if value.is_p2pkh() {
            Ok(Self::P2PKH)
        } else {
            Err(eyre!("Not compatible script"))
        }
    }
}

impl TryFrom<Slice> for Addresstype {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        match value.read_u8() {
            x if x == Addresstype::P2PK as u8 => Ok(Addresstype::P2PK),
            x if x == Addresstype::P2PKH as u8 => Ok(Addresstype::P2PKH),
            _ => Err(eyre!("Unknown type")),
        }
    }
}
impl From<Addresstype> for Slice {
    fn from(addresstype: Addresstype) -> Self {
        (addresstype as u8).to_be_bytes().into()
    }
}
