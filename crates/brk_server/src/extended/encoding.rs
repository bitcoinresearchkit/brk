use axum::{
    body::Bytes,
    http::{header, HeaderMap, HeaderValue},
};

/// HTTP content encoding for pre-compressed caching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEncoding {
    Brotli,
    Gzip,
    Zstd,
    Identity,
}

impl ContentEncoding {
    /// Negotiate the best encoding from the Accept-Encoding header.
    /// Priority: br > zstd > gzip > identity.
    pub fn negotiate(headers: &HeaderMap) -> Self {
        let accept = match headers.get(header::ACCEPT_ENCODING) {
            Some(v) => v,
            None => return Self::Identity,
        };
        let s = match accept.to_str() {
            Ok(s) => s,
            Err(_) => return Self::Identity,
        };

        let mut best = Self::Identity;
        for part in s.split(',') {
            let name = part.split(';').next().unwrap_or("").trim();
            match name {
                "br" => return Self::Brotli,
                "zstd" => best = Self::Zstd,
                "gzip" if matches!(best, Self::Identity) => best = Self::Gzip,
                _ => {}
            }
        }
        best
    }

    /// Compress bytes with this encoding. Identity returns bytes unchanged.
    pub fn compress(self, bytes: Bytes) -> Bytes {
        match self {
            Self::Identity => bytes,
            Self::Brotli => {
                use std::io::Write;
                let mut output = Vec::with_capacity(bytes.len() / 2);
                {
                    let mut writer = brotli::CompressorWriter::new(&mut output, 4096, 4, 22);
                    writer.write_all(&bytes).expect("brotli compression failed");
                }
                Bytes::from(output)
            }
            Self::Gzip => {
                use flate2::write::GzEncoder;
                use std::io::Write;
                let mut encoder = GzEncoder::new(
                    Vec::with_capacity(bytes.len() / 2),
                    flate2::Compression::new(3),
                );
                encoder.write_all(&bytes).expect("gzip compression failed");
                Bytes::from(encoder.finish().expect("gzip finish failed"))
            }
            Self::Zstd => Bytes::from(
                zstd::encode_all(bytes.as_ref(), 3).expect("zstd compression failed"),
            ),
        }
    }

    /// Wire name used for Content-Encoding header and cache key suffix.
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Brotli => "br",
            Self::Gzip => "gzip",
            Self::Zstd => "zstd",
            Self::Identity => "identity",
        }
    }

    #[inline]
    pub(crate) fn header_value(self) -> Option<HeaderValue> {
        match self {
            Self::Identity => None,
            _ => Some(HeaderValue::from_static(self.as_str())),
        }
    }
}
