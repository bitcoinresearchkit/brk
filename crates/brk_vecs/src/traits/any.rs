use crate::{Stamp, Version};

pub fn i64_to_usize(i: i64, len: usize) -> usize {
    if i >= 0 {
        (i as usize).min(len)
    } else {
        let v = len as i64 + i;
        if v < 0 { 0 } else { v as usize }
    }
}

pub trait AnyVec: Send + Sync {
    fn version(&self) -> Version;
    fn name(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn index_type_to_string(&self) -> &'static str;
    fn value_type_to_size_of(&self) -> usize;
    fn etag(&self, stamp: Stamp, to: Option<i64>) -> String {
        let len = self.len();
        format!(
            "{}-{}-{}",
            to.map_or(len, |to| {
                if to.is_negative() {
                    len.checked_sub(to.unsigned_abs() as usize)
                        .unwrap_or_default()
                } else {
                    to as usize
                }
            }),
            u64::from(self.version()),
            u64::from(stamp),
        )
    }

    #[inline]
    fn i64_to_usize(&self, i: i64) -> usize {
        let len = self.len();
        i64_to_usize(i, len)
    }
}
