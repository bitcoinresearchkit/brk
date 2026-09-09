use serde_json::json;

use super::*;

#[test]
fn schema_order_matches_sorted_queue_for_every_three_node_graph() {
    let names = ["A", "B", "É"];
    for mask in 0..512 {
        let mut schemas = TypeSchemas::default();
        let mut dependencies = [Vec::new(), Vec::new(), Vec::new()];
        let mut degrees = [0usize; 3];
        for (i, name) in names.iter().enumerate() {
            let mut refs = vec![json!({"$ref": "#/components/schemas/Missing"})];
            for (j, dependency) in names.iter().enumerate() {
                if mask & (1 << (i * 3 + j)) != 0 {
                    refs.push(json!({"$ref": format!("#/components/schemas/{dependency}")}));
                    if i != j {
                        dependencies[i].push(j);
                        degrees[j] += 1;
                    }
                }
            }
            schemas.insert(name.to_string(), json!({"anyOf": refs}));
        }
        let mut queue: Vec<_> = (0..3).filter(|&i| degrees[i] == 0).collect();
        let mut expected = Vec::new();
        while let Some(i) = queue.pop() {
            expected.push(i);
            for &j in &dependencies[i] {
                degrees[j] -= 1;
                if degrees[j] == 0 {
                    queue.push(j);
                    queue.sort();
                }
            }
        }
        expected.reverse();
        for i in 0..3 {
            if !expected.contains(&i) {
                expected.push(i);
            }
        }
        assert_eq!(
            topological_sort_schemas(&schemas),
            expected.into_iter().map(|i| names[i]).collect::<Vec<_>>(),
            "graph {mask}"
        );
    }
    assert!(topological_sort_schemas(&TypeSchemas::default()).is_empty());
}
