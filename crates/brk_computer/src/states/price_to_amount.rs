use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use bincode::{Decode, Encode, config, decode_from_std_read, encode_into_std_write};
use brk_core::{Dollars, Height, Result, Sats};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::states::SupplyState;

#[derive(Clone, Debug)]
pub struct PriceToAmount {
    pathbuf: PathBuf,
    height: Option<Height>,
    state: State,
}

#[derive(Clone, Default, Debug, Deref, DerefMut, Serialize, Deserialize, Encode, Decode)]
struct State(BTreeMap<Dollars, Sats>);

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

        let config = config::standard();
        let file = File::open(Self::path_state_(&path))?;
        let mut reader = BufReader::new(file);
        let state = decode_from_std_read(&mut reader, config)?;

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

        let file = File::create(self.path_state()).inspect_err(|_| {
            dbg!(self.path_state());
        })?;
        let mut writer = BufWriter::new(file);
        encode_into_std_write(&self.state, &mut writer, config::standard())?;

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
