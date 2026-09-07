use std::{cell::Cell, marker::PhantomData, rc::Rc, sync::Arc};

use bitview_traversable::{IndexMap, Traversable, TreeNode};
use vecdb::{AnyExportableVec, ReadOnlyClone, Ro, Rw, StorageMode};

// Deliberately not Clone, Default, or ReadOnlyClone.
struct WriterCache(Vec<u64>);

#[derive(bitview_traversable_derive::Traversable)]
struct Projection<M: StorageMode = Rw> {
    #[traversable(skip)]
    mode: PhantomData<M>,
    #[traversable(skip, rename = "not_visible")]
    shared: Arc<u64>,
    writer: M::WriteOnly<WriterCache>,
    #[traversable(skip)]
    legacy_optional: Option<u64>,
}

#[derive(bitview_traversable_derive::Traversable)]
struct OnlyWriter<T, Mode: StorageMode = Rw> {
    state: <Mode as StorageMode>::WriteOnly<T>,
}

#[derive(bitview_traversable_derive::Traversable)]
struct TupleWriter<M: StorageMode = Rw>(M::WriteOnly<WriterCache>);

#[derive(bitview_traversable_derive::Traversable)]
#[traversable(transparent)]
struct TransparentWriter<M: StorageMode = Rw> {
    state: M::WriteOnly<WriterCache>,
}

#[derive(Clone)]
struct Visible;

impl Traversable for Visible {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::branch(IndexMap::from([(
            "visible".into(),
            TreeNode::branch(IndexMap::new()),
        )]))
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::empty()
    }
}

#[derive(bitview_traversable_derive::Traversable)]
#[traversable(transparent)]
struct TransparentMixed<M: StorageMode = Rw> {
    state: M::WriteOnly<WriterCache>,
    visible: Visible,
}

#[test]
fn transparent_projection_delegates_past_writer_only_state() {
    let writer = TransparentMixed::<Rw> {
        state: WriterCache(vec![1]),
        visible: Visible,
    };
    let reader = writer.read_only_clone();
    for tree in [writer.to_tree_node(), reader.to_tree_node()] {
        let TreeNode::Branch(branch) = tree else {
            panic!("expected visible branch")
        };
        assert_eq!(
            branch.keys().map(String::as_str).collect::<Vec<_>>(),
            ["visible"]
        );
    }
    assert_eq!(writer.state.0, [1]);
    let (): () = reader.state;
}

#[test]
fn writer_only_payloads_need_no_traits_and_are_not_consumed_by_projection() {
    struct OnDrop(Rc<Cell<usize>>);
    impl Drop for OnDrop {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drops = Rc::new(Cell::new(0));
    let writer = OnlyWriter::<_, Rw> {
        state: OnDrop(drops.clone()),
    };
    let reader: OnlyWriter<OnDrop, Ro> = writer.read_only_clone();
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    assert_send_sync(&reader);
    assert_eq!(size_of_val(&reader), 0);
    let (): () = reader.state;
    assert_eq!(drops.get(), 0);
    assert_eq!(writer.state.0.get(), 0);
    drop(writer);
    assert_eq!(drops.get(), 1);
}

#[test]
fn writer_only_tuple_and_transparent_structs_have_empty_traversal() {
    let tuple = TupleWriter::<Rw>(WriterCache(vec![1]));
    let transparent = TransparentWriter::<Rw> {
        state: WriterCache(vec![2]),
    };
    assert_eq!(tuple.iter_any_exportable().count(), 0);
    assert_eq!(transparent.iter_any_exportable().count(), 0);
    assert_eq!(size_of_val(&tuple.read_only_clone()), 0);
    assert_eq!(size_of_val(&transparent.read_only_clone()), 0);
    assert_eq!(tuple.0.0, [1]);
    assert_eq!(transparent.state.0, [2]);
}

#[test]
fn projection_omits_writer_state_but_preserves_shared_handles() {
    let writer = Projection::<Rw> {
        mode: PhantomData,
        shared: Arc::new(42),
        writer: WriterCache(vec![1, 2, 3]),
        legacy_optional: Some(7),
    };
    let reader: Projection<Ro> = writer.read_only_clone();

    assert_eq!(writer.writer.0, [1, 2, 3]);
    let (): () = reader.writer;
    assert!(Arc::ptr_eq(&writer.shared, &reader.shared));
    assert_eq!(writer.legacy_optional, Some(7));
    assert_eq!(reader.legacy_optional, None);
    assert_eq!(writer.iter_any_exportable().count(), 0);
    assert_eq!(reader.iter_any_exportable().count(), 0);
}
