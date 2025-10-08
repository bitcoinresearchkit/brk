use crate::BlkPosition;

#[derive(Debug)]
pub struct BlkMetadata {
    position: BlkPosition,
    len: u32,
}

impl BlkMetadata {
    pub fn new(position: BlkPosition, len: u32) -> Self {
        Self { position, len }
    }
    pub fn position(&self) -> BlkPosition {
        self.position
    }

    pub fn blk_index(&self) -> u16 {
        self.position.blk_index()
    }

    pub fn offset(&self) -> u32 {
        self.position.offset()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.len
    }
}
