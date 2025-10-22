use brk_structs::{BlockHash, Height};

pub enum BlockRange {
    Span { start: Height, end: Height },
    Start { start: Height },
    End { end: Height },
    Last { n: u32 },
    After { hash: BlockHash },
}
