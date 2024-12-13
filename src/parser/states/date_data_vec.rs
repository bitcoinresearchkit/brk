use allocative::Allocative;
use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::structs::{BlockData, BlockPath, DateData};

use super::AnyState;

#[derive(Default, Deref, DerefMut, Debug, Serialize, Deserialize, Encode, Decode, Allocative)]
pub struct DateDataVec(Vec<DateData>);

impl DateDataVec {
    pub fn last_block(&self) -> Option<&BlockData> {
        self.iter().flat_map(|date_data| &date_data.blocks).last()
    }

    pub fn last_mut_block(&mut self) -> Option<&mut BlockData> {
        self.iter_mut()
            .flat_map(|date_data| &mut date_data.blocks)
            .last()
    }

    pub fn second_last_block(&self) -> Option<&BlockData> {
        self.iter()
            .flat_map(|date_data| &date_data.blocks)
            .rev()
            .nth(1)
    }

    pub fn get_date_data(&self, block_path: &BlockPath) -> Option<&DateData> {
        self.0.get(block_path.date_index as usize)
    }

    pub fn get_block_data(&self, block_path: &BlockPath) -> Option<&BlockData> {
        self.0
            .get(block_path.date_index as usize)
            .and_then(|date_data| date_data.blocks.get(block_path.block_index as usize))
    }
}

impl AnyState for DateDataVec {
    fn name<'a>() -> &'a str {
        "date_data_vec"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
