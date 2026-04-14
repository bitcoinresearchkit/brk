use std::ops::ControlFlow;

use brk_types::{BlkMetadata, BlkPosition};

use crate::{XORBytes, XORIndex, xor_bytes::XOR_LEN};

const MAGIC_BYTES: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xD9];

/// Returns the position **immediately after** the matched magic, or
/// `None` if no match. Advances `xor_i` by the bytes consumed either
/// way.
pub(crate) fn find_magic(bytes: &[u8], xor_i: &mut XORIndex, xor_bytes: XORBytes) -> Option<usize> {
    let len = bytes.len();
    if len < MAGIC_BYTES.len() {
        xor_i.add_assign(len);
        return None;
    }

    let xb = *xor_bytes;
    let mut phase = xor_i.phase();
    let mut i = 0;
    let stop = len - MAGIC_BYTES.len();

    while i <= stop {
        if bytes[i] ^ xb[phase] == MAGIC_BYTES[0] {
            let p1 = (phase + 1) & (XOR_LEN - 1);
            let p2 = (phase + 2) & (XOR_LEN - 1);
            let p3 = (phase + 3) & (XOR_LEN - 1);
            if bytes[i + 1] ^ xb[p1] == MAGIC_BYTES[1]
                && bytes[i + 2] ^ xb[p2] == MAGIC_BYTES[2]
                && bytes[i + 3] ^ xb[p3] == MAGIC_BYTES[3]
            {
                xor_i.set_phase(phase + MAGIC_BYTES.len());
                return Some(i + MAGIC_BYTES.len());
            }
        }
        phase = (phase + 1) & (XOR_LEN - 1);
        i += 1;
    }

    xor_i.set_phase(phase + (len - i));
    None
}

/// Scans `buf` (the full contents of one blk file) for blocks,
/// calling `on_block` for each. The block bytes are passed as a
/// mutable borrow so the callback can clone (to ship to a parser
/// thread) or process in place (to peek the header).
pub(crate) fn scan_bytes(
    buf: &mut [u8],
    blk_index: u16,
    xor_bytes: XORBytes,
    mut on_block: impl FnMut(BlkMetadata, &mut [u8], XORIndex) -> ControlFlow<()>,
) {
    let mut xor_i = XORIndex::default();
    let mut i = 0;

    while let Some(off) = find_magic(&buf[i..], &mut xor_i, xor_bytes) {
        i += off;
        if i + 4 > buf.len() {
            return;
        }
        let mut size_bytes = [buf[i], buf[i + 1], buf[i + 2], buf[i + 3]];
        xor_i.bytes(&mut size_bytes, xor_bytes);
        let len = u32::from_le_bytes(size_bytes) as usize;
        i += 4;
        if i + len > buf.len() {
            return;
        }
        let metadata = BlkMetadata::new(BlkPosition::new(blk_index, i as u32), len as u32);
        if on_block(metadata, &mut buf[i..i + len], xor_i).is_break() {
            return;
        }
        i += len;
        xor_i.add_assign(len);
    }
}
