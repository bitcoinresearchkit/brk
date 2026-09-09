use std::slice;

use super::BytesStrategy;
use crate::{BytesVecValue, ValueStrategy, variants::raw::RawStrategy};

impl<T: BytesVecValue> RawStrategy<T> for BytesStrategy<T> {
    #[inline(always)]
    unsafe fn read_from_ptr(ptr: *const u8, byte_offset: usize) -> T {
        unsafe {
            if T::IS_NATIVE_LAYOUT {
                (ptr.add(byte_offset) as *const T).read_unaligned()
            } else {
                let slice = slice::from_raw_parts(ptr.add(byte_offset), size_of::<T>());
                <BytesStrategy<T> as ValueStrategy<T>>::read(slice).unwrap_unchecked()
            }
        }
    }
}
