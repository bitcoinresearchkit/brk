use brk_error::Result;
use brk_types::{CentsCompact, Sats, UrpdRaw};

use super::{AgeRangeUrpds, EncodedAgeRangeUrpds};

impl EncodedAgeRangeUrpds {
    /// The selected encoded sections in their aggregation order.
    pub fn sections(&self) -> impl Iterator<Item = &[u8]> {
        self.id.age_range_ids().iter().map(|id| {
            let range = id.select(&self.ranges);
            &self.data[range.start - self.start..range.end - self.start]
        })
    }

    pub fn decode(&self) -> Result<UrpdRaw> {
        Ok(UrpdRaw {
            map: self.decode_entries()?.into_iter().collect(),
        })
    }

    /// Decode and validate the selected aggregate without building a map.
    pub fn decode_entries(&self) -> Result<Vec<(CentsCompact, Sats)>> {
        // Decode one section at a time, including All, so captured requests do
        // not retain the expanded maps for every age range simultaneously.
        let entries = self.sections().try_fold(Vec::new(), |left, section| {
            let right = UrpdRaw::deserialize_entries(section)?;
            if left.is_empty() {
                return Ok(right);
            }
            if right.is_empty() {
                return Ok(left);
            }
            AgeRangeUrpds::merge_sorted(&left, &right)
        })?;
        UrpdRaw::checked_entry_supply(&entries)?;
        Ok(entries)
    }
}
