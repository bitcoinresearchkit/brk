use canopydb::{Tree as CanopyTree, TreeOptions, WriteTransaction};

use super::{Database, Tree};

#[derive(Debug)]
pub struct Transaction<'a, K, V> {
    tx: WriteTransaction,
    tree: Tree<'a, K, V>,
}

impl<'a, K, V> Transaction<'a, K, V> {
    pub fn new(db: &Database) -> color_eyre::Result<Self> {
        let tx = db.begin_write()?;

        let tree = Tree::new(&tx)?;

        Ok(Self { tx, tree })
    }
}
