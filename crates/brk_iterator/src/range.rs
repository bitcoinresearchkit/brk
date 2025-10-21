use brk_structs::Height;

pub enum BlockRange {
    Span { start: Height, end: Height },
    Start { start: Height },
    End { end: Height },
    Last { n: u32 },
}
