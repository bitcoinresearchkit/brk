use biter::bitcoin::ScriptBuf;
use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::Addresstype;

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addressbytes(Slice);

impl TryFrom<(&ScriptBuf, Addresstype)> for Addressbytes {
    type Error = color_eyre::Report;
    fn try_from(tuple: (&ScriptBuf, Addresstype)) -> Result<Self, Self::Error> {
        let (script, addresstype) = tuple;

        match addresstype {
            Addresstype::P2PK => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    67 => &bytes[1..66],
                    35 => &bytes[1..34],
                    _ => {
                        dbg!(bytes);
                        return Err(eyre!("Wrong len"));
                    }
                };
                Ok(Self(bytes.into()))
            }
            Addresstype::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self(bytes.into()))
            }
            Addresstype::P2SH => {
                let bytes = &script.as_bytes()[2..22];
                Ok(Self(bytes.into()))
            }
            Addresstype::P2WPKH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self(bytes.into()))
            }
            Addresstype::P2WSH => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self(bytes.into()))
            }
            Addresstype::P2TR => {
                let bytes = &script.as_bytes()[2..];
                Ok(Self(bytes.into()))
            }
            Addresstype::Multisig => Err(eyre!("multisig address type")),
            Addresstype::PushOnly => Err(eyre!("push_only address type")),
            Addresstype::Unknown => Err(eyre!("unknown address type")),
            Addresstype::Empty => Err(eyre!("empty address type")),
            Addresstype::OpReturn => Err(eyre!("op_return address type")),
        }
    }
}

impl From<Slice> for Addressbytes {
    fn from(value: Slice) -> Self {
        Self(value)
    }
}
impl From<&Addressbytes> for Slice {
    fn from(value: &Addressbytes) -> Self {
        value.0.clone()
    }
}
