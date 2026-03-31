use std::ops::ControlFlow;

use brk_types::{BlkMetadata, BlkPosition};

use crate::{XORBytes, XORIndex};

const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];

pub fn find_magic(bytes: &[u8], xor_i: &mut XORIndex, xor_bytes: XORBytes) -> Option<usize> {
    let mut window = [0u8; 4];
    for (i, &b) in bytes.iter().enumerate() {
        window.rotate_left(1);
        window[3] = xor_i.byte(b, xor_bytes);
        if window == MAGIC_BYTES {
            return Some(i + 1);
        }
    }
    None
}

pub struct ScanResult {
    pub first_magic: Option<usize>,
    pub interrupted: bool,
}

/// Scans `buf` for blocks. `file_offset` is the absolute position of `buf[0]` in the file.
/// Calls `on_block` for each complete block found.
pub fn scan_bytes(
    buf: &mut [u8],
    blk_index: u16,
    file_offset: usize,
    xor_bytes: XORBytes,
    mut on_block: impl FnMut(BlkMetadata, Vec<u8>, XORIndex) -> ControlFlow<()>,
) -> ScanResult {
    let mut xor_i = XORIndex::default();
    xor_i.add_assign(file_offset);
    let mut first_magic = None;
    let mut i = 0;

    while let Some(off) = find_magic(&buf[i..], &mut xor_i, xor_bytes) {
        let before = i;
        i += off;
        first_magic.get_or_insert(before + off.saturating_sub(4));
        if i + 4 > buf.len() {
            break;
        }
        let len = u32::from_le_bytes(
            xor_i
                .bytes(&mut buf[i..i + 4], xor_bytes)
                .try_into()
                .unwrap(),
        ) as usize;
        i += 4;
        if i + len > buf.len() {
            break;
        }
        let position = BlkPosition::new(blk_index, (file_offset + i) as u32);
        let metadata = BlkMetadata::new(position, len as u32);
        if on_block(metadata, buf[i..i + len].to_vec(), xor_i).is_break() {
            return ScanResult {
                first_magic,
                interrupted: true,
            };
        }
        i += len;
        xor_i.add_assign(len);
    }

    ScanResult {
        first_magic,
        interrupted: false,
    }
}
