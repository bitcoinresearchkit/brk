use std::{
    collections::BTreeMap,
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_structs::{Dollars, Height, Sats};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, IntoBytes};

use crate::states::SupplyState;

#[derive(Clone, Debug)]
pub struct PriceToAmount {
    pathbuf: PathBuf,
    height: Option<Height>,
    state: State,
}

impl PriceToAmount {
    pub fn forced_import(path: &Path, name: &str) -> Self {
        Self::import(path, name).unwrap_or_else(|_| Self {
            pathbuf: Self::path_(path, name),
            height: None,
            state: State::default(),
        })
    }

    pub fn import(path: &Path, name: &str) -> Result<Self> {
        let path = Self::path_(path, name);
        fs::create_dir_all(&path)?;

        let state = State::deserialize(&fs::read(&path)?)?;

        Ok(Self {
            height: Height::try_from(Self::path_height_(&path).as_path()).ok(),
            pathbuf: path,
            state,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Dollars, &Sats)> {
        self.state.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn first_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.first_key_value()
    }

    pub fn last_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.last_key_value()
    }

    pub fn increment(&mut self, price: Dollars, supply_state: &SupplyState) {
        *self.state.entry(price).or_default() += supply_state.value;
    }

    pub fn decrement(&mut self, price: Dollars, supply_state: &SupplyState) {
        let amount = self.state.get_mut(&price).unwrap();
        *amount -= supply_state.value;
        if *amount == Sats::ZERO {
            self.state.remove(&price);
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        self.state.clear();
        self.height = None;
        fs::remove_dir_all(&self.pathbuf)?;
        fs::create_dir_all(&self.pathbuf)?;
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.height = Some(height);
        height.write(&self.path_height())?;
        fs::write(self.path_state(), self.state.serialize())?;
        Ok(())
    }

    pub fn height(&self) -> Option<Height> {
        self.height
    }

    fn path_(path: &Path, name: &str) -> PathBuf {
        path.join(format!("{name}_price_to_amount"))
    }

    fn path_state(&self) -> PathBuf {
        Self::path_state_(&self.pathbuf)
    }
    fn path_state_(path: &Path) -> PathBuf {
        path.join("state")
    }

    fn path_height(&self) -> PathBuf {
        Self::path_height_(&self.pathbuf)
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }
}

#[derive(Clone, Default, Debug, Deref, DerefMut, Serialize, Deserialize)]
struct State(BTreeMap<Dollars, Sats>);

impl State {
    fn serialize(&self) -> Vec<u8> {
        let len = self.len();

        let mut buffer = Vec::with_capacity(8 + len * 16);

        buffer.extend_from_slice(len.as_bytes());

        self.iter().for_each(|(key, value)| {
            buffer.extend_from_slice(key.as_bytes());
            buffer.extend_from_slice(value.as_bytes());
        });

        buffer
    }

    fn deserialize(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);
        let mut buffer = [0u8; 8];

        cursor
            .read_exact(&mut buffer)
            .map_err(|_| Error::Str("Failed to read entry count"))?;
        let entry_count = usize::read_from_bytes(&buffer)?;

        let mut map = BTreeMap::new();

        for _ in 0..entry_count {
            cursor.read_exact(&mut buffer)?;
            let key = Dollars::read_from_bytes(&buffer)?;

            cursor.read_exact(&mut buffer)?;
            let value = Sats::read_from_bytes(&buffer)?;

            map.insert(key, value);
        }

        Ok(Self(map))
    }
}
