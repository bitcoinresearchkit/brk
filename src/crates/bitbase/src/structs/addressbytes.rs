use biter::bitcoin::ScriptBuf;
use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::{Addressindex, Addresstype};

#[derive(Debug, Deref, DerefMut)]
pub struct Addressbytes(Slice);

impl TryFrom<(&ScriptBuf, Addresstype, Addressindex)> for Addressbytes {
    type Error = color_eyre::Report;
    fn try_from(tuple: (&ScriptBuf, Addresstype, Addressindex)) -> Result<Self, Self::Error> {
        let (script, addresstype, addressindex) = tuple;

        match addresstype {
            Addresstype::P2PK => {
                let bytes = script.as_bytes();
                let bytes = match bytes.len() {
                    67 => &script.as_bytes()[1..66],
                    35 => &script.as_bytes()[1..34],
                    _ => {
                        dbg!(bytes);
                        return Err(eyre!("Wrong len"));
                    }
                };

                if bytes[0] != 4 {
                    dbg!(bytes);
                    return Err(eyre!("Doesn't start with a 4"));
                }

                Ok(Self(bytes.into()))
            }
            Addresstype::P2PKH => {
                let bytes = &script.as_bytes()[3..23];
                Ok(Self(bytes.into()))
            }
            _ => {
                if script.is_p2sh() {
                    Err(eyre!("p2sh address type"))
                } else if script.is_p2wpkh() {
                    Err(eyre!("p2wpkh address type"))
                } else if script.is_p2wsh() {
                    Err(eyre!("p2wsh address type"))
                } else if script.is_p2tr() {
                    Err(eyre!("p2tr address type"))
                } else if script.is_empty() {
                    Err(eyre!("empty address type"))
                } else if script.is_op_return() {
                    Err(eyre!("op_return address type"))
                } else if script.is_multisig() {
                    Err(eyre!("multisig address type"))
                } else if script.is_push_only() {
                    Err(eyre!("push only address type"))
                } else {
                    Ok(Self(addressindex.into()))
                }
            }
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
