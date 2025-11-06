#![doc = include_str!("../README.md")]

pub use vecdb::{CheckedSub, Exit, PrintableIndex, Version};

use brk_error::{Error, Result};

mod address;
mod addressbytes;
mod addressbyteshash;
mod addresschainstats;
mod addressmempoolstats;
mod addressstats;
mod addresstypeaddressindexoutpoint;
mod addresstypeaddressindextxindex;
mod anyaddressindex;
mod bitcoin;
mod blkmetadata;
mod blkposition;
mod block;
mod blockhash;
mod blockhashprefix;
mod bytes;
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
mod format;
mod halvingepoch;
mod health;
mod height;
mod index;
mod indexinfo;
mod limit;
mod loadedaddressdata;
mod loadedaddressindex;
mod metric;
mod metriccount;
mod metrics;
mod monthindex;
mod ohlc;
mod opreturnindex;
mod outpoint;
mod outputtype;
mod p2aaddressindex;
mod p2abytes;
mod p2msoutputindex;
mod p2pk33addressindex;
mod p2pk33bytes;
mod p2pk65addressindex;
mod p2pk65bytes;
mod p2pkhaddressindex;
mod p2pkhbytes;
mod p2shaddressindex;
mod p2shbytes;
mod p2traddressindex;
mod p2trbytes;
mod p2wpkhaddressindex;
mod p2wpkhbytes;
mod p2wshaddressindex;
mod p2wshbytes;
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
mod tx;
mod txid;
mod txidpath;
mod txidprefix;
mod txin;
mod txindex;
mod txinindex;
mod txout;
mod txoutindex;
mod txstatus;
mod txversion;
mod typeindex;
mod unit;
mod unknownoutputindex;
mod vin;
mod vout;
mod weekindex;
mod weight;
mod yearindex;

pub use address::*;
pub use addressbytes::*;
pub use addressbyteshash::*;
pub use addresschainstats::*;
pub use addressmempoolstats::*;
pub use addressstats::*;
pub use addresstypeaddressindexoutpoint::*;
pub use addresstypeaddressindextxindex::*;
pub use anyaddressindex::*;
pub use bitcoin::*;
pub use blkmetadata::*;
pub use blkposition::*;
pub use block::*;
pub use blockhash::*;
pub use blockhashprefix::*;
pub use bytes::*;
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
pub use format::*;
pub use halvingepoch::*;
pub use health::*;
pub use height::*;
pub use index::*;
pub use indexinfo::*;
pub use limit::*;
pub use loadedaddressdata::*;
pub use loadedaddressindex::*;
pub use metric::*;
pub use metriccount::*;
pub use metrics::*;
pub use monthindex::*;
pub use ohlc::*;
pub use opreturnindex::*;
pub use outpoint::*;
pub use outputtype::*;
pub use p2aaddressindex::*;
pub use p2abytes::*;
pub use p2msoutputindex::*;
pub use p2pk33addressindex::*;
pub use p2pk33bytes::*;
pub use p2pk65addressindex::*;
pub use p2pk65bytes::*;
pub use p2pkhaddressindex::*;
pub use p2pkhbytes::*;
pub use p2shaddressindex::*;
pub use p2shbytes::*;
pub use p2traddressindex::*;
pub use p2trbytes::*;
pub use p2wpkhaddressindex::*;
pub use p2wpkhbytes::*;
pub use p2wshaddressindex::*;
pub use p2wshbytes::*;
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
pub use tx::*;
pub use txid::*;
pub use txidpath::*;
pub use txidprefix::*;
pub use txin::*;
pub use txindex::*;
pub use txinindex::*;
pub use txout::*;
pub use txoutindex::*;
pub use txstatus::*;
pub use txversion::*;
pub use typeindex::*;
pub use unit::*;
pub use unknownoutputindex::*;
pub use vin::*;
pub use vout::*;
pub use weekindex::*;
pub use weight::*;
pub use yearindex::*;

#[allow(clippy::result_unit_err)]
pub fn copy_first_2bytes(slice: &[u8]) -> Result<[u8; 2]> {
    let mut buf: [u8; 2] = [0; 2];
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
