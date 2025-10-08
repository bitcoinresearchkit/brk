#![doc = include_str!("../README.md")]

pub use vecdb::{CheckedSub, Exit, PrintableIndex, Version};

use brk_error::{Error, Result};

mod addressbytes;
mod addressbyteshash;
mod anyaddressindex;
mod bitcoin;
mod blkmetadata;
mod blkposition;
mod block;
mod blockhash;
mod blockhashprefix;
mod cents;
mod date;
mod dateindex;
mod decadeindex;
mod difficultyepoch;
mod dollars;
mod emptyaddressdata;
mod emptyaddressindex;
mod emptyoutputindex;
mod feerate;
mod halvingepoch;
mod height;
mod index;
mod indexinfo;
mod inputindex;
mod loadedaddressdata;
mod loadedaddressindex;
mod monthindex;
mod ohlc;
mod opreturnindex;
mod outputindex;
mod outputtype;
mod p2aaddressindex;
mod p2msoutputindex;
mod p2pk33addressindex;
mod p2pk65addressindex;
mod p2pkhaddressindex;
mod p2shaddressindex;
mod p2traddressindex;
mod p2wpkhaddressindex;
mod p2wshaddressindex;
mod pool;
mod poolid;
mod pools;
mod quarterindex;
mod rawlocktime;
mod sats;
mod semesterindex;
mod stored_bool;
mod stored_f32;
mod stored_f64;
mod stored_i16;
mod stored_string;
mod stored_u16;
mod stored_u32;
mod stored_u64;
mod stored_u8;
mod timestamp;
mod treenode;
mod txid;
mod txidprefix;
mod txindex;
mod txversion;
mod typeindex;
mod typeindex_with_outputindex;
mod unit;
mod unknownoutputindex;
mod vin;
mod vout;
mod weekindex;
mod weight;
mod yearindex;

pub use addressbytes::*;
pub use addressbyteshash::*;
pub use anyaddressindex::*;
pub use bitcoin::*;
pub use blkmetadata::*;
pub use blkposition::*;
pub use block::*;
pub use blockhash::*;
pub use blockhashprefix::*;
pub use cents::*;
pub use date::*;
pub use dateindex::*;
pub use decadeindex::*;
pub use difficultyepoch::*;
pub use dollars::*;
pub use emptyaddressdata::*;
pub use emptyaddressindex::*;
pub use emptyoutputindex::*;
pub use feerate::*;
pub use halvingepoch::*;
pub use height::*;
pub use index::*;
pub use indexinfo::*;
pub use inputindex::*;
pub use loadedaddressdata::*;
pub use loadedaddressindex::*;
pub use monthindex::*;
pub use ohlc::*;
pub use opreturnindex::*;
pub use outputindex::*;
pub use outputtype::*;
pub use p2aaddressindex::*;
pub use p2msoutputindex::*;
pub use p2pk33addressindex::*;
pub use p2pk65addressindex::*;
pub use p2pkhaddressindex::*;
pub use p2shaddressindex::*;
pub use p2traddressindex::*;
pub use p2wpkhaddressindex::*;
pub use p2wshaddressindex::*;
pub use pool::*;
pub use poolid::*;
pub use pools::*;
pub use quarterindex::*;
pub use rawlocktime::*;
pub use sats::*;
pub use semesterindex::*;
pub use stored_bool::*;
pub use stored_f32::*;
pub use stored_f64::*;
pub use stored_i16::*;
pub use stored_string::*;
pub use stored_u8::*;
pub use stored_u16::*;
pub use stored_u32::*;
pub use stored_u64::*;
pub use timestamp::*;
pub use treenode::*;
pub use txid::*;
pub use txidprefix::*;
pub use txindex::*;
pub use txversion::*;
pub use typeindex::*;
pub use typeindex_with_outputindex::*;
pub use unit::*;
pub use unknownoutputindex::*;
pub use vin::*;
pub use vout::*;
pub use weekindex::*;
pub use weight::*;
pub use yearindex::*;

#[allow(clippy::result_unit_err)]
pub fn copy_first_4bytes(slice: &[u8]) -> Result<[u8; 4]> {
    let mut buf: [u8; 4] = [0; 4];
    let buf_len = buf.len();
    if slice.len() < buf_len {
        return Err(Error::Str("Buffer is too small to convert to 8 bytes"));
    }
    slice.iter().take(buf_len).enumerate().for_each(|(i, r)| {
        buf[i] = *r;
    });
    Ok(buf)
}

#[allow(clippy::result_unit_err)]
pub fn copy_first_8bytes(slice: &[u8]) -> Result<[u8; 8]> {
    let mut buf: [u8; 8] = [0; 8];
    let buf_len = buf.len();
    if slice.len() < buf_len {
        return Err(Error::Str("Buffer is too small to convert to 8 bytes"));
    }
    slice.iter().take(buf_len).enumerate().for_each(|(i, r)| {
        buf[i] = *r;
    });
    Ok(buf)
}
