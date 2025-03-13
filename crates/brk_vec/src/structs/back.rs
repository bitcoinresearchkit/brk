use std::{fs::File, sync::OnceLock};

use super::CompressedPagesMetadata;

type CompressedPage<T> = Option<(usize, Box<[T]>)>;

pub enum Back<T> {
    Raw {
        raw_pages: Vec<OnceLock<Box<memmap2::Mmap>>>,
        raw_page: memmap2::Mmap,
        file: File,
        file_position: u64,
        buf: Vec<u8>,
    },
    Compressed {
        decoded_pages: Option<Vec<OnceLock<Box<[T]>>>>,
        decoded_page: CompressedPage<T>,
        pages: CompressedPagesMetadata,
    },
}
