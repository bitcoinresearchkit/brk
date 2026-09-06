pub mod bytes;

/// Storage format selection for stored vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    /// Explicit byte serialization with little-endian byte order.
    /// **PORTABLE** across different endianness systems. Uses custom Bytes trait.
    Bytes,
    /// Direct memory mapping with native byte order via zerocopy.
    /// **NOT PORTABLE** - fastest but endianness-dependent. Best for random access.
    ZeroCopy,
    /// Pcodec compression optimized for numeric sequences (best compression for numbers).
    Pco = 64,
    /// LZ4 compression (fastest compression/decompression, moderate ratio).
    LZ4 = 65,
    /// Zstd compression (highest compression ratio, slower).
    Zstd = 66,
}

impl Format {
    #[inline]
    pub fn is_raw(&self) -> bool {
        matches!(self, Self::ZeroCopy | Self::Bytes)
    }

    #[inline]
    pub fn is_compressed(&self) -> bool {
        matches!(self, Self::Pco | Self::LZ4 | Self::Zstd)
    }

    #[inline]
    pub fn is_zerocopy(&self) -> bool {
        *self == Self::ZeroCopy
    }

    #[inline]
    pub fn is_bytes(&self) -> bool {
        *self == Self::Bytes
    }

    #[inline]
    pub fn is_pcodec(&self) -> bool {
        *self == Self::Pco
    }

    #[inline]
    pub fn is_lz4(&self) -> bool {
        *self == Self::LZ4
    }

    #[inline]
    pub fn is_zstd(&self) -> bool {
        *self == Self::Zstd
    }
}
