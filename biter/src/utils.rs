use std::{collections::BTreeMap, fs, path::PathBuf, time::UNIX_EPOCH};

const BLK: &str = "blk";
const DAT: &str = ".dat";

pub fn scan_blocks_dir(data_dir_path: &str) -> BTreeMap<usize, PathBuf> {
    let blocks_dir_path = &format!("{data_dir_path}/blocks");

    fs::read_dir(blocks_dir_path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let is_file = path.is_file();

            if is_file {
                let file_name = path.file_name().unwrap().to_str().unwrap();

                file_name.starts_with(BLK) && file_name.ends_with(DAT)
            } else {
                false
            }
        })
        .map(|path| {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let blk_index = file_name[BLK.len()..(file_name.len() - DAT.len())]
                .parse::<usize>()
                .unwrap();

            (blk_index, path)
        })
        .collect::<BTreeMap<_, _>>()
}

pub fn path_to_modified_time(path: &PathBuf) -> u64 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
