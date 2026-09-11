use std::sync::atomic::{AtomicUsize, Ordering};

use vecdb::{
    AnyStoredVec, BytesVec, Database, ImportableVec, ReadableBoxedVec, ReadableCloneableVec,
    Version, WritableVec,
};

pub fn mapping(
    db: &Database,
    values: impl IntoIterator<Item = usize>,
) -> ReadableBoxedVec<usize, usize> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let name = format!("mapping_{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let mut source = BytesVec::<usize, usize>::import(db, &name, Version::ONE).unwrap();
    for value in values {
        source.push(value);
    }
    source.write().unwrap();
    source.read_only_boxed_clone()
}
