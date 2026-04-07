use bitcoin::{absolute::LockTime, locktime::absolute::LOCK_TIME_THRESHOLD};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

/// Transaction locktime. Values below 500,000,000 are interpreted as block heights; values at or above are Unix timestamps.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pco, JsonSchema)]
#[schemars(example = 0, example = 840000, example = 840001, example = 1713571200, example = 4294967295_u32)]
pub struct RawLockTime(u32);

impl From<LockTime> for RawLockTime {
    #[inline]
    fn from(value: LockTime) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<RawLockTime> for LockTime {
    #[inline]
    fn from(value: RawLockTime) -> Self {
        let value = value.0;
        if value < LOCK_TIME_THRESHOLD {
            bitcoin::locktime::absolute::Height::from_consensus(value)
                .unwrap()
                .into()
        } else {
            bitcoin::locktime::absolute::Time::from_consensus(value)
                .unwrap()
                .into()
        }
    }
}

impl std::fmt::Display for RawLockTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lock_time = LockTime::from(*self);
        f.write_str(&lock_time.to_string())
    }
}

impl Formattable for RawLockTime {
    fn write_to(&self, buf: &mut Vec<u8>) {
        use std::fmt::Write;
        let mut s = String::new();
        write!(s, "{}", self).unwrap();
        buf.extend_from_slice(s.as_bytes());
    }

    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        let start = f.len();
        self.fmt_into(f);
        if f.as_bytes()[start..].contains(&b',') {
            f.insert(start, '"');
            f.push('"');
        }
        Ok(())
    }

    fn fmt_json(&self, buf: &mut Vec<u8>) {
        buf.push(b'"');
        self.write_to(buf);
        buf.push(b'"');
    }
}
