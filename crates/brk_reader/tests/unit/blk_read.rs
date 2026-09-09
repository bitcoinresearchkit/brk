use std::fs;

use brk_rpc::{Auth, Client};
use tempfile::tempdir;

use super::*;
use crate::Reader;

#[test]
fn streaming_and_direct_reads_preserve_every_xor_phase() {
    let directory = tempdir().unwrap();
    let plain: Vec<u8> = (0..200).collect();
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    for mask in [[0; 8], [1, 17, 33, 49, 65, 81, 97, 113]] {
        fs::write(directory.path().join("xor.dat"), mask).unwrap();
        let encoded: Vec<u8> = plain
            .iter()
            .enumerate()
            .map(|(index, byte)| byte ^ mask[index % 8])
            .collect();
        fs::write(directory.path().join("blk00000.dat"), encoded).unwrap();
        let reader = Reader::new_without_rlimit(directory.path().to_owned(), &client);
        for offset in 0..8 {
            let position = BlkPosition::new(0, offset as u32);
            let mut stream = reader.reader_at(position).unwrap();
            let mut bytes = [0; 90];
            let mut begin = 0;
            for count in [1, 7, 79, 3] {
                stream.read_exact(&mut bytes[begin..begin + count]).unwrap();
                begin += count;
            }
            assert_eq!(bytes, plain[offset..offset + 90]);
            assert_eq!(reader.read_raw_bytes(position, 90).unwrap(), bytes);
            let mut rest = Vec::new();
            stream.read_to_end(&mut rest).unwrap();
            assert_eq!(rest, plain[offset + 90..]);
        }
    }
}
