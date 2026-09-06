// Copyright (c) 2024-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use byteview::{Builder, ByteView};

use super::Slice;

pub trait SliceExt: Sized {
    fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self;
    fn fused(left: &[u8], right: &[u8]) -> Self;
}

impl SliceExt for Slice {
    fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self {
        Self(self.0.slice(range))
    }

    fn fused(left: &[u8], right: &[u8]) -> Self {
        Self(ByteView::fused(left, right))
    }
}

impl Slice {
    /// Construct a [`Slice`] from a byte slice.
    #[must_use]
    pub fn new(bytes: &[u8]) -> Self {
        Self(bytes.into())
    }

    /// Returns the bytes slice containing the entire data.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self
    }

    #[doc(hidden)]
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[doc(hidden)]
    #[must_use]
    pub unsafe fn builder_unzeroed(len: usize) -> Builder {
        unsafe { ByteView::builder_unzeroed(len) }
    }

    #[doc(hidden)]
    pub fn from_reader<R: std::io::Read>(reader: &mut R, len: usize) -> std::io::Result<Self> {
        ByteView::from_reader(reader, len).map(Self)
    }
}

// Arc::from<Vec<u8>> is specialized
impl From<Vec<u8>> for Slice {
    fn from(value: Vec<u8>) -> Self {
        Self(ByteView::from(value))
    }
}

// Arc::from<Vec<String>> is specialized
impl From<String> for Slice {
    fn from(value: String) -> Self {
        Self(ByteView::from(value.into_bytes()))
    }
}

impl From<ByteView> for Slice {
    fn from(value: ByteView) -> Self {
        Self(value)
    }
}

impl From<Slice> for ByteView {
    fn from(value: Slice) -> Self {
        value.0
    }
}
