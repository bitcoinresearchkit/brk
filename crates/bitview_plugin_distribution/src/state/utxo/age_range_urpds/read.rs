use std::{
    fs::File,
    io,
    ops::Range,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

use bitview_cohort::{AgeRange, AgeRangeId, UTXOAggregateId};
use brk_error::Result;
use brk_types::{CentsCompact, Date, Sats, UrpdRaw};

use super::{AgeRangeUrpds, DIR_NAME, HEADER_LEN};

#[path = "encoded.rs"]
mod encoded;

/// Owned compressed input for one aggregate, captured under publication protection.
pub struct EncodedAgeRangeUrpds {
    data: Vec<u8>,
    ranges: AgeRange<Range<usize>>,
    start: usize,
    id: UTXOAggregateId,
}

impl AgeRangeUrpds {
    pub fn dir(states_path: &Path) -> PathBuf {
        states_path.join(DIR_NAME)
    }

    pub fn path(states_path: &Path, date: Date) -> PathBuf {
        Self::dir(states_path).join(date.to_string())
    }

    pub fn read(states_path: &Path, date: Date) -> Result<Self> {
        let data = Self::read_bytes(states_path, date)?;
        let ranges = Self::ranges(&data, data.len())?;
        let entries = AgeRange::try_from_fn(|id| {
            UrpdRaw::deserialize_entries(&data[id.select(&ranges).clone()])
        })?;
        Ok(Self { entries })
    }

    fn read_bytes(states_path: &Path, date: Date) -> Result<Vec<u8>> {
        let path = Self::path(states_path, date);
        let data = UrpdRaw::read_encoded_file(&path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot read age-range URPD '{}': {error}", path.display()),
            )
        })?;
        Ok(data)
    }

    /// Read one encoded section without decompressing it or loading other cohorts.
    /// Callers must hold the producer's publication guard during the file read.
    pub fn read_one_bytes(states_path: &Path, id: AgeRangeId, date: Date) -> Result<Vec<u8>> {
        let path = Self::path(states_path, date);
        let (file, ranges) = Self::open(&path)?;
        let range = id.select(&ranges);
        let mut data = vec![0; range.len()];
        file.read_exact_at(&mut data, range.start as u64)?;
        Ok(data)
    }

    /// Capture encoded aggregate input while holding the producer's publication guard.
    pub fn read_aggregate_encoded(
        states_path: &Path,
        id: UTXOAggregateId,
        date: Date,
    ) -> Result<EncodedAgeRangeUrpds> {
        if id == UTXOAggregateId::All {
            let data = Self::read_bytes(states_path, date)?;
            let ranges = Self::ranges(&data, data.len())?;
            return Ok(EncodedAgeRangeUrpds {
                data,
                ranges,
                start: 0,
                id,
            });
        }

        let path = Self::path(states_path, date);
        let (file, ranges) = Self::open(&path)?;
        let ids = id.age_range_ids();
        debug_assert!(
            ids.windows(2)
                .all(|pair| pair[0].index() + 1 == pair[1].index())
        );
        let first = ids.first().expect("aggregate contains an age range");
        let last = ids.last().expect("aggregate contains an age range");
        let start = first.select(&ranges).start;
        let end = last.select(&ranges).end;
        let mut data = vec![0; end - start];
        file.read_exact_at(&mut data, start as u64)?;

        Ok(EncodedAgeRangeUrpds {
            data,
            ranges,
            start,
            id,
        })
    }

    fn open(path: &Path) -> Result<(File, AgeRange<Range<usize>>)> {
        let file = File::open(path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot read age-range URPD '{}': {error}", path.display()),
            )
        })?;
        let mut header = [0; HEADER_LEN];
        let file_len = file.metadata()?.len();
        if file_len > UrpdRaw::MAX_ENCODED_BYTES as u64 {
            return Err(Self::invalid("file exceeds snapshot limit"));
        }
        file.read_exact_at(&mut header, 0)?;
        let ranges = Self::ranges(&header, file_len as usize)?;
        Ok((file, ranges))
    }

    pub fn get(&self, id: AgeRangeId) -> &[(CentsCompact, Sats)] {
        id.select(&self.entries)
    }
}
