use crate::{Error, Result};

#[allow(clippy::result_unit_err)]
pub fn copy_first_4bytes(slice: &[u8]) -> Result<[u8; 4]> {
    let mut buf: [u8; 4] = [0; 4];
    let buf_len = buf.len();
    if slice.len() < buf_len {
        return Err(Error::String("Buffer is too small to convert to 8 bytes"));
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
        return Err(Error::String("Buffer is too small to convert to 8 bytes"));
    }
    slice.iter().take(buf_len).enumerate().for_each(|(i, r)| {
        buf[i] = *r;
    });
    Ok(buf)
}
