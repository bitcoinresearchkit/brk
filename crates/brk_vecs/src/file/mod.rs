use std::{
    fs::{self, OpenOptions},
    os::unix::io::AsRawFd,
    path::{Path, PathBuf},
    sync::Arc,
};

use libc::off_t;
use log::info;
use memmap2::{MmapMut, MmapOptions};
use parking_lot::{RwLock, RwLockReadGuard};

mod identifier;
mod layout;
mod reader;
mod region;
mod regions;

pub use identifier::*;
use layout::*;
use rayon::prelude::*;
pub use reader::*;
pub use region::*;
use regions::*;

use crate::{Error, Result};

pub const PAGE_SIZE: u64 = 4096;
pub const PAGE_SIZE_MINUS_1: u64 = PAGE_SIZE - 1;

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    regions: RwLock<Regions>,
    layout: RwLock<Layout>,
    file: RwLock<fs::File>,
    mmap: RwLock<MmapMut>,
}

impl File {
    pub fn open(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)?;

        let regions = Regions::open(path)?;

        let layout = Layout::from(&regions);

        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(Self::data_path_(path))?;

        let mmap = Self::create_mmap(&file)?;

        Ok(Self {
            path: path.to_owned(),
            file: RwLock::new(file),
            mmap: RwLock::new(mmap),
            regions: RwLock::new(regions),
            layout: RwLock::new(layout),
        })
    }

    pub fn file_len(&self) -> Result<u64> {
        Ok(self.file.read().metadata()?.len())
    }

    pub fn set_min_len(&self, len: u64) -> Result<()> {
        let len = Self::ceil_number_to_page_size_multiple(len);

        let file_len = self.file_len()?;
        if file_len < len {
            let mut mmap = self.mmap.write();
            let file = self.file.write();
            file.set_len(len)?;
            *mmap = Self::create_mmap(&file)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn set_min_regions(&self, regions: usize) -> Result<()> {
        self.regions
            .write()
            .set_min_len((regions * SIZE_OF_REGION) as u64)?;
        self.set_min_len(regions as u64 * PAGE_SIZE)
    }

    pub fn create_region_if_needed(&self, id: &str) -> Result<(usize, Arc<RwLock<Region>>)> {
        let regions = self.regions.read();
        if let Some(index) = regions.get_region_index_from_id(id) {
            return Ok((index, regions.get_region_from_index(index).unwrap()));
        }
        drop(regions);

        let mut regions = self.regions.write();
        let mut layout = self.layout.write();

        let start = if let Some(start) = layout.find_smallest_adequate_hole(PAGE_SIZE) {
            layout.remove_or_compress_hole(start, PAGE_SIZE);
            start
        } else {
            let start = layout
                .get_last_region_index()
                .map(|index| {
                    let region_opt = regions.get_region_from_index(index);
                    let region = region_opt.as_ref().unwrap().read();
                    region.start() + region.reserved()
                })
                .unwrap_or_default();

            let len = start + PAGE_SIZE;

            self.set_min_len(len)?;

            start
        };

        let (index, region) = regions.create_region(id.to_owned(), start)?;

        layout.insert_region(start, index);

        Ok((index, region))
    }

    pub fn get_region(&self, identifier: Identifier) -> Result<RwLockReadGuard<'static, Region>> {
        let regions = self.regions.read();
        let region_opt = regions.get_region(identifier);
        let region_arc = region_opt.ok_or(Error::Str("Unknown region"))?;
        let region = region_arc.read();
        let region: RwLockReadGuard<'static, Region> = unsafe { std::mem::transmute(region) };
        Ok(region)
    }

    pub fn create_region_reader<'a>(&'a self, identifier: Identifier) -> Result<Reader<'a>> {
        let mmap: RwLockReadGuard<'a, MmapMut> = self.mmap.read();
        let region = self.get_region(identifier)?;
        Ok(Reader::new(mmap, region))
    }

    #[inline]
    pub fn write_all_to_region(&self, identifier: Identifier, data: &[u8]) -> Result<()> {
        self.write_all_to_region_at_(identifier, data, None)
    }

    #[inline]
    pub fn write_all_to_region_at(
        &self,
        identifier: Identifier,
        data: &[u8],
        at: u64,
    ) -> Result<()> {
        self.write_all_to_region_at_(identifier, data, Some(at))
    }

    fn write_all_to_region_at_(
        &self,
        identifier: Identifier,
        data: &[u8],
        at: Option<u64>,
    ) -> Result<()> {
        let regions = self.regions.read();
        let Some(region_lock) = regions.get_region(identifier.clone()) else {
            return Err(Error::Str("Unknown region"));
        };

        let region_index = regions.identifier_to_index(identifier).unwrap();

        let region = region_lock.read();
        let start = region.start();
        let reserved = region.reserved();
        let len = region.len();
        drop(region);

        let data_len = data.len() as u64;
        let new_len = at.map_or(len + data_len, |at| (at + data_len).max(len));

        if at.is_some_and(|at| at >= start + reserved) {
            return Err(Error::Str("Invalid at parameter"));
        }

        // Write to reserved space if possible
        if new_len <= reserved {
            // println!(
            //     "Write to {region_index} reserved space at {}",
            //     start + at.unwrap_or(len)
            // );

            if at.is_none() {
                self.write(start + len, data);
            }

            let mut region = region_lock.write();
            if let Some(at) = at {
                self.write(start + at, data);
            }
            if len != new_len {
                region.set_len(new_len);
            }
            regions.write_to_mmap(&region, region_index);

            return Ok(());
        }

        assert!(new_len > reserved);
        let mut new_reserved = reserved;
        while new_len > new_reserved {
            new_reserved *= 2;
        }
        assert!(new_len <= new_reserved);
        let added_reserve = new_reserved - reserved;

        let mut layout = self.layout.write();

        // If is last continue writing
        if layout.is_last_anything(region_index) {
            // println!(
            //     "{region_index} Append to file at {}",
            //     start + at.unwrap_or(len)
            // );

            self.set_min_len(start + new_reserved)?;
            let mut region = region_lock.write();
            region.set_reserved(new_reserved);
            drop(region);
            drop(layout);

            self.write(start + len, data);

            let mut region = region_lock.write();
            region.set_len(new_len);
            regions.write_to_mmap(&region, region_index);

            return Ok(());
        }

        // Expand region to the right if gap is wide enough
        let hole_start = start + reserved;
        if layout
            .get_hole(hole_start)
            .is_some_and(|gap| gap >= added_reserve)
        {
            // println!("Expand {region_index} to hole");

            layout.remove_or_compress_hole(hole_start, added_reserve);
            let mut region = region_lock.write();
            region.set_reserved(new_reserved);
            drop(region);
            drop(layout);

            self.write(start + len, data);

            let mut region = region_lock.write();
            region.set_len(new_len);
            regions.write_to_mmap(&region, region_index);

            return Ok(());
        }

        // Find hole big enough to move the region
        if let Some(hole_start) = layout.find_smallest_adequate_hole(new_reserved) {
            // println!("Move {region_index} to hole at {hole_start}");

            layout.remove_or_compress_hole(hole_start, new_reserved);
            drop(layout);

            self.write(
                hole_start,
                &self.mmap.read()[start as usize..(start + len) as usize],
            );
            self.write(hole_start + len, data);

            let mut region = region_lock.write();
            let mut layout = self.layout.write();
            layout.move_region(hole_start, region_index, &region)?;
            drop(layout);

            region.set_start(hole_start);
            region.set_reserved(new_reserved);
            region.set_len(new_len);
            regions.write_to_mmap(&region, region_index);

            return Ok(());
        }

        let new_start = layout.len(&regions);
        // Write at the end
        // println!(
        //     "Move {region_index} to the end, from {start}..{} to {new_start}..{}",
        //     start + reserved,
        //     new_start + new_reserved
        // );
        self.set_min_len(new_start + new_reserved)?;
        layout.reserve(new_start, new_reserved);
        drop(layout);

        self.write(
            new_start,
            &self.mmap.read()[start as usize..(start + len) as usize],
        );
        self.write(new_start + len, data);

        let mut region = region_lock.write();
        let mut layout = self.layout.write();
        layout.move_region(new_start, region_index, &region)?;
        drop(layout);

        region.set_start(new_start);
        region.set_reserved(new_reserved);
        region.set_len(new_len);
        regions.write_to_mmap(&region, region_index);

        Ok(())
    }

    fn write(&self, at: u64, data: &[u8]) {
        let mmap = self.mmap.read();

        let data_len = data.len();
        let start = at as usize;
        let end = start + data_len;

        if end > mmap.len() {
            unreachable!("Trying to write beyond mmap")
        }

        let slice = unsafe { std::slice::from_raw_parts_mut(mmap.as_ptr() as *mut u8, mmap.len()) };

        slice[start..end].copy_from_slice(data);
    }

    /// From relative to start
    pub fn truncate_region(&self, identifier: Identifier, from: u64) -> Result<()> {
        let Some(region) = self.regions.read().get_region(identifier) else {
            return Err(Error::Str("Unknown region"));
        };
        let mut region_ = region.write();
        let start = region_.start();
        let len = region_.len();
        let reserved = region_.reserved();

        // dbg!(from, start);

        if from == len {
            return Ok(());
        } else if from > len {
            return Err(Error::Str("Truncating further than length"));
        }

        region_.set_len(from);

        let end = start + reserved;
        let start = Self::ceil_number_to_page_size_multiple(start + from);
        if start > end {
            unreachable!("Should not be possible");
        } else if start < end {
            self.punch_hole(start, end - start)?;
        }
        Ok(())
    }

    pub fn remove_region(&self, identifier: Identifier) -> Result<Option<Arc<RwLock<Region>>>> {
        let mut regions = self.regions.write();

        let mut layout = self.layout.write();

        let index_opt = regions.identifier_to_index(identifier.clone());

        let Some(region) = regions.remove_region(identifier)? else {
            return Ok(None);
        };

        let index = index_opt.unwrap();

        let region_ = region.write();

        layout.remove_region(index, &region_)?;

        self.punch_hole(region_.start(), region_.reserved())?;

        drop(region_);

        Ok(Some(region))
    }

    fn create_mmap(file: &fs::File) -> Result<MmapMut> {
        Ok(unsafe { MmapOptions::new().map_mut(file)? })
    }

    pub fn regions(&self) -> RwLockReadGuard<'_, Regions> {
        self.regions.read()
    }

    pub fn layout(&self) -> RwLockReadGuard<'_, Layout> {
        self.layout.read()
    }

    pub fn mmap(&self) -> RwLockReadGuard<'_, MmapMut> {
        self.mmap.read()
    }

    fn ceil_number_to_page_size_multiple(num: u64) -> u64 {
        (num + PAGE_SIZE_MINUS_1) & !PAGE_SIZE_MINUS_1
    }

    fn data_path(&self) -> PathBuf {
        Self::data_path_(&self.path)
    }
    fn data_path_(path: &Path) -> PathBuf {
        path.join("data")
    }

    pub fn disk_usage(&self) -> String {
        let path = self.data_path();

        let output = std::process::Command::new("du")
            .arg("-h")
            .arg(&path)
            .output()
            .expect("Failed to run du");

        String::from_utf8_lossy(&output.stdout)
            .replace(path.to_str().unwrap(), " ")
            .trim()
            .to_string()
    }

    pub fn flush(&self) -> Result<()> {
        let mmap = self.mmap.read();
        let regions = self.regions.read();
        mmap.flush()?;
        regions.flush()
    }

    pub fn punch_holes(&self) -> Result<()> {
        let file = self.file.write();
        let mmap = self.mmap.read();
        let layout = self.layout.read();
        Self::punch_holes_(&file, &mmap, &layout)
    }

    fn punch_holes_(file: &fs::File, mmap: &MmapMut, layout: &Layout) -> Result<()> {
        layout
            .start_to_hole()
            .par_iter()
            .try_for_each(|(&start, &hole)| -> Result<()> {
                assert!(start % PAGE_SIZE == 0);
                assert!(hole % PAGE_SIZE == 0);
                let has_old_data =
                    mmap[start as usize] != 0 || mmap[(start + hole - PAGE_SIZE) as usize] != 0;
                if has_old_data {
                    info!("Punching a hole of {hole} bytes at {start}...");
                    Self::punch_hole_(file, start, hole)
                } else {
                    Ok(())
                }
            })
    }

    fn punch_hole(&self, start: u64, length: u64) -> Result<()> {
        let file = self.file.write();
        Self::punch_hole_impl(&file, start, length)
    }

    fn punch_hole_(file: &fs::File, start: u64, length: u64) -> Result<()> {
        Self::punch_hole_impl(file, start, length)
    }

    #[cfg(target_os = "macos")]
    fn punch_hole_impl(file: &fs::File, start: u64, length: u64) -> Result<()> {
        let fpunchhole = FPunchhole {
            fp_flags: 0,
            reserved: 0,
            fp_offset: start as libc::off_t,
            fp_length: length as libc::off_t,
        };

        let result = unsafe {
            libc::fcntl(
                file.as_raw_fd(),
                libc::F_PUNCHHOLE,
                &fpunchhole as *const FPunchhole,
            )
        };

        if result == -1 {
            let err = std::io::Error::last_os_error();
            return Err(Error::String(format!("Failed to punch hole: {err}")));
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn punch_hole_impl(file: &fs::File, start: u64, length: u64) -> Result<()> {
        let result = unsafe {
            libc::fallocate(
                file.as_raw_fd(),
                libc::FALLOC_FL_PUNCH_HOLE | libc::FALLOC_FL_KEEP_SIZE,
                start as libc::off_t,
                length as libc::off_t,
            )
        };

        if result == -1 {
            let err = std::io::Error::last_os_error();
            return Err(Error::String(format!("Failed to punch hole: {err}")));
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    fn punch_hole_impl(_file: &fs::File, _start: u64, _length: u64) -> Result<()> {
        Err(Error::String(
            "Hole punching not supported on this platform".to_string(),
        ))
    }
}

#[repr(C)]
struct FPunchhole {
    fp_flags: u32,
    reserved: u32,
    fp_offset: off_t,
    fp_length: off_t,
}
