use bitview_cohort::{Amount, CohortContext, UTXOAndAddrGroups, UTXOGroups};

#[test]
fn mapping_visits_both_independent_axes_with_unambiguous_names() {
    let groups = UTXOAndAddrGroups {
        utxo: UTXOGroups::new(|_, _| 7),
        addr_balance: Amount::new(|_, _| 11),
    };
    let mut visited = Vec::new();
    let mapped = groups.map_named(|context, filter, name, value| {
        let name = context.full_name(filter, name);
        visited.push((context, name.clone(), *value));
        name
    });
    let utxo_len = groups.utxo.iter().count();
    let addr_len = groups.addr_balance.iter().count();
    assert_eq!(visited.len(), utxo_len + addr_len);
    assert!(
        visited[..utxo_len]
            .iter()
            .all(|(context, _, value)| { *context == CohortContext::Utxo && *value == 7 })
    );
    assert!(visited[utxo_len..].iter().all(|(context, name, value)| {
        *context == CohortContext::Addr && name.starts_with("addrs_") && *value == 11
    }));
    for (utxo, addr) in mapped
        .utxo
        .utxo_amount
        .iter()
        .zip(mapped.addr_balance.iter())
    {
        assert_eq!(utxo.strip_prefix("utxos_"), addr.strip_prefix("addrs_"));
    }
    let names: std::collections::BTreeSet<_> = visited.iter().map(|(_, name, _)| name).collect();
    assert_eq!(names.len(), visited.len());
}

#[cfg(feature = "storage")]
mod storage {
    use super::*;
    use bitview_traversable::{IndexMap, Traversable, TreeNode};
    use vecdb::{AnyExportableVec, ReadOnlyClone, Ro, Rw, StorageMode};

    // Clone-only lazy views must not need ReadOnlyClone to project their owner.
    #[derive(Clone)]
    struct View;

    impl Traversable for View {
        fn to_tree_node(&self) -> TreeNode {
            TreeNode::branch(IndexMap::new())
        }

        fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
            std::iter::empty()
        }
    }

    #[derive(Traversable)]
    struct AddressStorage<M: StorageMode = Rw> {
        #[traversable(flatten)]
        series: Amount<View>,
        writer: M::WriteOnly<Vec<u64>>,
    }

    #[test]
    fn storage_projection_preserves_flattened_paths_and_drops_writer_state() {
        let groups = UTXOAndAddrGroups {
            utxo: UTXOGroups::new(|_, _| View),
            addr_balance: AddressStorage::<Rw> {
                series: Amount::new(|_, _| View),
                writer: vec![1, 2, 3],
            },
        };
        let reader: UTXOAndAddrGroups<View, AddressStorage<Ro>> = groups.read_only_clone();
        let (): () = reader.addr_balance.writer;
        assert_eq!(groups.addr_balance.writer, [1, 2, 3]);
        let TreeNode::Branch(utxo) = groups.utxo.to_tree_node() else {
            panic!()
        };
        let expected: Vec<_> = utxo
            .keys()
            .map(String::as_str)
            .chain(["addr_balance"])
            .collect();
        for tree in [groups.to_tree_node(), reader.to_tree_node()] {
            let TreeNode::Branch(branch) = tree else {
                panic!()
            };
            assert_eq!(
                branch.keys().map(String::as_str).collect::<Vec<_>>(),
                expected
            );
            let TreeNode::Branch(addr) = &branch["addr_balance"] else {
                panic!()
            };
            assert_eq!(
                addr.keys().map(String::as_str).collect::<Vec<_>>(),
                ["range", "under", "over"]
            );
            assert!(!branch.contains_key("utxo"));
        }
    }
}
