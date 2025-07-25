use memmap2::MmapMut;
use parking_lot::RwLockReadGuard;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{File, PAGE_SIZE, Reader};

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct Region {
    /// Must be multiple of 4096
    start: u64,
    len: u64,
    /// Must be multiple of 4096, greater or equal to len
    reserved: u64,
}

pub const SIZE_OF_REGION: usize = size_of::<Region>();

impl Region {
    pub fn new(start: u64, len: u64, reserved: u64) -> Self {
        assert!(start % PAGE_SIZE == 0);
        assert!(reserved >= PAGE_SIZE);
        assert!(reserved % PAGE_SIZE == 0);
        assert!(len <= reserved);

        Self {
            start,
            len,
            reserved,
        }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn set_start(&mut self, start: u64) {
        assert!(start % PAGE_SIZE == 0);
        self.start = start
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn set_len(&mut self, len: u64) {
        assert!(len <= self.reserved());
        self.len = len
    }

    pub fn reserved(&self) -> u64 {
        self.reserved
    }

    pub fn set_reserved(&mut self, reserved: u64) {
        assert!(self.len() <= reserved);
        assert!(reserved >= PAGE_SIZE);
        assert!(reserved % PAGE_SIZE == 0);

        self.reserved = reserved;
    }

    pub fn left(&self) -> u64 {
        self.reserved - self.len
    }
}

pub trait RegionReader {
    fn create_reader(self, file: &File) -> Reader;
}

impl<'a> RegionReader for RwLockReadGuard<'a, Region> {
    fn create_reader(self, file: &File) -> Reader<'static> {
        let region: RwLockReadGuard<'static, Region> = unsafe { std::mem::transmute(self) };
        let mmap: RwLockReadGuard<'static, MmapMut> =
            unsafe { std::mem::transmute(file.mmap.read()) };
        Reader::new(mmap, region)
    }
}
