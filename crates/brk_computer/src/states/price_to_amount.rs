use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{Dollars, Height, Sats};
use derive_deref::{Deref, DerefMut};
use pco::standalone::{simple_decompress, simpler_compress};
use serde::{Deserialize, Serialize};
use vecdb::Bytes;

use crate::{states::SupplyState, utils::OptionExt};

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

    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No price state found at or before height".into(),
        ))?;
        self.state = Some(State::deserialize(&fs::read(path)?)?);
        Ok(height)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Dollars, &Sats)> {
        self.state.u().iter()
    }

    /// Iterate over entries in a price range with custom bounds.
    pub fn range<R: std::ops::RangeBounds<Dollars>>(
        &self,
        range: R,
    ) -> impl Iterator<Item = (&Dollars, &Sats)> {
        self.state.u().range(range)
    }

    pub fn is_empty(&self) -> bool {
        self.state.u().is_empty()
    }

    pub fn first_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.u().first_key_value()
    }

    pub fn last_key_value(&self) -> Option<(&Dollars, &Sats)> {
        self.state.u().last_key_value()
    }

    pub fn increment(&mut self, price: Dollars, supply_state: &SupplyState) {
        *self.state.um().entry(price).or_default() += supply_state.value;
    }

    pub fn decrement(&mut self, price: Dollars, supply_state: &SupplyState) {
        if let Some(amount) = self.state.um().get_mut(&price) {
            *amount -= supply_state.value;
            if *amount == Sats::ZERO {
                self.state.um().remove(&price);
            }
        } else {
            dbg!(price, &self.pathbuf);
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

    fn read_dir(&self, keep_only_before: Option<Height>) -> Result<BTreeMap<Height, PathBuf>> {
        Ok(fs::read_dir(&self.pathbuf)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let name = path.file_name()?.to_str()?;
                let height_str = name.strip_prefix(STATE_AT_).unwrap_or(name);
                if let Ok(h) = height_str.parse::<u32>().map(Height::from) {
                    if keep_only_before.is_none_or(|height| h < height) {
                        Some((h, path))
                    } else {
                        let _ = fs::remove_file(path);
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<BTreeMap<Height, PathBuf>>())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        let files = self.read_dir(Some(height))?;

        for (_, path) in files
            .iter()
            .take(files.len().saturating_sub(STATE_TO_KEEP - 1))
        {
            fs::remove_file(path)?;
        }

        fs::write(self.path_state(height), self.state.u().serialize()?)?;

        Ok(())
    }

    fn path_state(&self, height: Height) -> PathBuf {
        Self::path_state_(&self.pathbuf, height)
    }
    fn path_state_(path: &Path, height: Height) -> PathBuf {
        path.join(u32::from(height).to_string())
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
        buffer.extend(keys.len().to_bytes());
        buffer.extend(compressed_keys.len().to_bytes());
        buffer.extend(compressed_keys);
        buffer.extend(compressed_values);

        Ok(buffer)
    }

    fn deserialize(data: &[u8]) -> vecdb::Result<Self> {
        let entry_count = usize::from_bytes(&data[0..8])?;
        let keys_len = usize::from_bytes(&data[8..16])?;

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
