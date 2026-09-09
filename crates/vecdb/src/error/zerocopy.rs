use zerocopy::error::{ConvertError, SizeError};

use super::Error;

impl<A, B, C> From<ConvertError<A, B, C>> for Error {
    fn from(_: ConvertError<A, B, C>) -> Self {
        Self::ZeroCopyError
    }
}

impl<A, B> From<SizeError<A, B>> for Error {
    fn from(_: SizeError<A, B>) -> Self {
        Self::ZeroCopyError
    }
}
