// use lru::LruCache;
// use std::num::NonZeroUsize;

// struct SmartBlkReader {
//     // LRU cache of recently accessed files (memory mapped)
//     mmap_cache: LruCache<String, memmap2::Mmap>,
//     // Fallback to direct file I/O for cache misses
//     max_cached_files: usize,
// }

// impl SmartBlkReader {
//     fn new(max_cached: usize) -> Self {
//         Self {
//             mmap_cache: LruCache::new(NonZeroUsize::new(max_cached).unwrap()),
//             max_cached_files: max_cached,
//         }
//     }

//     fn get_transaction(
//         &mut self,
//         file_path: &str,
//         offset: u64,
//         length: usize,
//     ) -> Result<Transaction, Box<dyn std::error::Error>> {
//         // Try cache first
//         if let Some(mmap) = self.mmap_cache.get(file_path) {
//             let tx_data = &mmap[offset as usize..(offset as usize + length)];
//             let mut cursor = std::io::Cursor::new(tx_data);
//             return Ok(bitcoin::consensus::Decodable::consensus_decode(
//                 &mut cursor,
//             )?);
//         }

//         // Cache miss - use direct I/O and potentially cache the file
//         let mut file = File::open(file_path)?;
//         file.seek(SeekFrom::Start(offset))?;

//         let mut buffer = vec![0u8; length];
//         file.read_exact(&mut buffer)?;

//         // Optionally add to cache (based on access patterns)
//         if self.should_cache_file(file_path) {
//             let file_for_mmap = File::open(file_path)?;
//             if let Ok(mmap) = unsafe { memmap2::MmapOptions::new().map(&file_for_mmap) } {
//                 self.mmap_cache.put(file_path.to_string(), mmap);
//             }
//         }

//         let mut cursor = std::io::Cursor::new(&buffer);
//         Ok(bitcoin::consensus::Decodable::consensus_decode(
//             &mut cursor,
//         )?)
//     }

//     fn should_cache_file(&self, _file_path: &str) -> bool {
//         // Implement logic: recent files, frequently accessed files, etc.
//         true
//     }
// }
