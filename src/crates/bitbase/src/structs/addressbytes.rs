use biter::bitcoin::ScriptBuf;
use color_eyre::eyre::eyre;
use fjall::Slice;

#[derive(Debug)]
pub struct Addressbytes(Slice);

impl Addressbytes {
    pub fn to_prefix_slice(&self) -> Slice {
        self.0[..8].into()
    }
}

impl TryFrom<&ScriptBuf> for Addressbytes {
    type Error = color_eyre::Report;
    fn try_from(script: &ScriptBuf) -> Result<Self, Self::Error> {
        if script.is_p2pk() {
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
        } else if script.is_p2pkh() {
            let bytes = &script.as_bytes()[3..23];

            Ok(Self(bytes.into()))
        } else {
            Err(eyre!("Unsupported address type"))
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
