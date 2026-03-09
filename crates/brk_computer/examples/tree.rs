use std::{env, fs, path::Path};

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_traversable::{Traversable, TreeNode};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let tmp = env::temp_dir().join("brk_tree_gen");
    fs::create_dir_all(&tmp)?;

    let indexer = Indexer::forced_import(&tmp)?;
    let computer = Computer::forced_import(&tmp, &indexer)?;

    let tree = TreeNode::Branch(
        [
            ("indexed".to_string(), indexer.vecs.to_tree_node()),
            ("computed".to_string(), computer.to_tree_node()),
        ]
        .into_iter()
        .collect(),
    )
    .merge_branches()
    .expect("Tree merge failed");

    let json = serde_json::to_string_pretty(&tree)?;

    let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tree.json");
    fs::write(&out_path, &json)?;
    eprintln!("Wrote {} bytes to {}", json.len(), out_path.display());

    fs::remove_dir_all(&tmp)?;

    Ok(())
}
