use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use brk_error::Result;
use brk_structs::{Dollars, Height, Sats};
use derive_deref::{Deref, DerefMut};
use pco::standalone::{simple_decompress, simpler_compress};
use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, IntoBytes};

use crate::states::SupplyState;

#[derive(Clone, Debug)]
pub struct PriceToAmount {
    pathbuf: PathBuf,
    state: Option<State>,
}

const STATE_AT_: &str = "state_at_";
const STATE_TO_KEEP: usize = 10;

impl PriceToAmount {
    pub fn create(path: &Path, name: &str) -> Self {
        Self {
            pathbuf: path.join(format!("{name}_price_to_amount")),
            state: None,
        }
    }

    pub fn import_at(&mut self, height: Height) -> Result<()> {
        self.state = Some(State::deserialize(&fs::read(self.path_state(height))?)?);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Dollars, &Sats)> {
        self.state.as_ref().unwrap().iter()
    }

    pub fn is_empty(&self) -> bool {
        self.state.as_ref().unwrap().is_empty()
    }

    pub fn first_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.as_ref().unwrap().first_key_value()
    }

    pub fn last_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.as_ref().unwrap().last_key_value()
    }

    pub fn increment(&mut self, price: Dollars, supply_state: &SupplyState) {
        *self.state.as_mut().unwrap().entry(price).or_default() += supply_state.value;
    }

    pub fn decrement(&mut self, price: Dollars, supply_state: &SupplyState) {
        if let Some(amount) = self.state.as_mut().unwrap().get_mut(&price) {
            *amount -= supply_state.value;
            if *amount == Sats::ZERO {
                self.state.as_mut().unwrap().remove(&price);
            }
        } else {
            dbg!(&self.state, price, &self.pathbuf);
            unreachable!();
        }
    }

    pub fn init(&mut self) {
        self.state.replace(State::default());
    }

    pub fn clean(&mut self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.pathbuf);
        fs::create_dir_all(&self.pathbuf)?;
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        let files: BTreeMap<Height, PathBuf> = fs::read_dir(&self.pathbuf)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let name = path.file_name()?.to_str()?;
                if let Some(height_str) = name.strip_prefix(STATE_AT_) {
                    if let Ok(h) = height_str.parse::<u64>().map(Height::from) {
                        if h < height {
                            Some((h, path))
                        } else {
                            let _ = fs::remove_file(path);
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (_, path) in files
            .iter()
            .take(files.len().saturating_sub(STATE_TO_KEEP - 1))
        {
            fs::remove_file(path)?;
        }

        fs::write(
            self.path_state(height),
            self.state.as_ref().unwrap().serialize()?,
        )?;

        Ok(())
    }

    fn path_state(&self, height: Height) -> PathBuf {
        Self::path_state_(&self.pathbuf, height)
    }
    fn path_state_(path: &Path, height: Height) -> PathBuf {
        path.join(format!("{STATE_AT_}{}", height))
    }
}

#[derive(Clone, Default, Debug, Deref, DerefMut, Serialize, Deserialize)]
struct State(BTreeMap<Dollars, Sats>);

const COMPRESSION_LEVEL: usize = 4;

impl State {
    fn serialize(&self) -> vecdb::Result<Vec<u8>> {
        let keys: Vec<f64> = self.keys().cloned().map(f64::from).collect();
        let values: Vec<u64> = self.values().cloned().map(u64::from).collect();

        let compressed_keys = simpler_compress(&keys, COMPRESSION_LEVEL)?;
        let compressed_values = simpler_compress(&values, COMPRESSION_LEVEL)?;

        let mut buffer = Vec::new();
        buffer.extend(keys.len().as_bytes());
        buffer.extend(compressed_keys.len().as_bytes());
        buffer.extend(compressed_keys);
        buffer.extend(compressed_values);

        Ok(buffer)
    }

    fn deserialize(data: &[u8]) -> vecdb::Result<Self> {
        let entry_count = usize::read_from_bytes(&data[0..8])?;
        let keys_len = usize::read_from_bytes(&data[8..16])?;

        let keys: Vec<f64> = simple_decompress(&data[16..16 + keys_len])?;
        let values: Vec<u64> = simple_decompress(&data[16 + keys_len..])?;

        let map: BTreeMap<Dollars, Sats> = keys
            .into_iter()
            .zip(values)
            .map(|(k, v)| (Dollars::from(k), Sats::from(v)))
            .collect();

        assert_eq!(map.len(), entry_count);

        Ok(Self(map))
    }
}
