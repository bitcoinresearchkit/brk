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
