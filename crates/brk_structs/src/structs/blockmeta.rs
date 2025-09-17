#[derive(Debug, Clone, Copy)]
pub struct BlockPosition {
    pub blk_index: u16,
    pub offset: usize,
    pub len: u32,
}
