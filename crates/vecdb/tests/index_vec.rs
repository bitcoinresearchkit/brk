use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering::Relaxed},
};

use vecdb::{AnyVec, IndexVec, ReadOnlyClone, ReadableVec, TypedVec, Version};

// Deliberately not readable: IndexVec must require only metadata.
#[derive(Clone)]
struct Metadata(Arc<AtomicUsize>);

impl AnyVec for Metadata {
    fn name(&self) -> &str {
        "metadata"
    }
    fn version(&self) -> Version {
        Version::ONE
    }
    fn len(&self) -> usize {
        self.0.load(Relaxed)
    }
    fn index_type_to_string(&self) -> &'static str {
        "usize"
    }
    fn value_type_to_string(&self) -> &'static str {
        "u64"
    }
    fn value_type_to_size_of(&self) -> usize {
        size_of::<u64>()
    }
    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }
}

impl TypedVec for Metadata {
    type I = usize;
    type T = u64;
}

#[test]
fn generates_indices_and_constants_using_only_live_metadata() {
    let len = Arc::new(AtomicUsize::new(5));
    let values = IndexVec::new("indices", Version::ONE, Metadata(len.clone()), |i| {
        i as u64 + 1
    });
    let constants = IndexVec::new("constant", Version::ONE, Metadata(len.clone()), |_| 42u64);
    let cloned = values.read_only_clone();
    assert_eq!(values.name(), "indices");
    assert_eq!(values.version(), Version::ONE + Version::ONE);
    assert!(values.region_names().is_empty());
    assert_eq!(values.collect(), [1, 2, 3, 4, 5]);
    assert_eq!(constants.collect(), [42; 5]);
    assert_eq!(values.collect_range_at(3, 100), [4, 5]);
    assert!(values.collect_range_at(5, 100).is_empty());
    assert!(values.collect_range_at(4, 2).is_empty());
    let mut out = vec![99];
    values.read_into_at(1, 3, &mut out);
    assert_eq!(out, [99, 2, 3]);
    values.read_sorted_into_at(&[0, 0, 4, 5, 9], &mut out);
    assert_eq!(out, [99, 2, 3, 1, 1, 5]);
    let mut chunks = Vec::new();
    values.for_each_chunk_at(2, 100, &mut |at, values| {
        assert_eq!(at, 2 + chunks.len());
        chunks.extend_from_slice(values);
    });
    assert_eq!(chunks, [3, 4, 5]);
    let mut visited = Vec::new();
    values.for_each_range_dyn_at(1, 3, &mut |value| visited.push(value));
    assert_eq!(visited, [2, 3]);
    assert_eq!(values.fold_range_at(0, 100, 0, |sum, v| sum + v), 15);
    let mut visits = 0;
    let result = values.try_fold_range_at(0, 100, 0, |sum, v| {
        visits += 1;
        if v == 3 { Err(v) } else { Ok(sum + v) }
    });
    assert_eq!(result, Err(3));
    assert_eq!(visits, 3);
    len.store(6, Relaxed);
    assert_eq!(cloned.collect_one_at(5), Some(6));
    assert_eq!(constants.collect_one_at(5), Some(42));
    len.store(2, Relaxed);
    assert_eq!(cloned.collect(), [1, 2]);
    assert_eq!(constants.collect(), [42; 2]);
    assert_eq!(values.collect_one_at(2), None);
    len.store(0, Relaxed);
    assert!(values.collect().is_empty());
    assert!(constants.collect().is_empty());
}
