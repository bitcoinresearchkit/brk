// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io,
    path::{Component, Path, PathBuf},
    result::Result,
};

use sanakirja::btree::{page, Db_};
pub use sanakirja::*;

use crate::{DatabaseKey, DatabaseValue};

pub type UnitDatabase = Base<(), ()>;

///
/// A simple wrapper around Sanakirja aatabase that acts as a very fast on disk BTreeMap.
///
/// The state of the tree is uncommited until `.export()` is called during which it is unsafe to stop the program.
///
pub struct Base<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    pathbuf: PathBuf,
    db: Db_<Key, Value, page::Page<Key, Value>>,
    txn: MutTxn<Env, ()>,
}

const ROOT_DB: usize = 0;
const PAGE_SIZE: u64 = 4096;

const DEFRAGMENT_RATIO_THRESHOLD: f64 = 0.5;

impl<Key, Value> Base<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    const KEY_SIZE: usize = size_of::<Key>();
    const VALUE_SIZE: usize = size_of::<Value>();
    const KEY_AND_VALUE_SIZE: usize = Self::KEY_SIZE + Self::VALUE_SIZE;

    /// Open a database without a lock file where only one instance is safe to open.
    pub fn open(pathbuf: PathBuf) -> Result<Self, Error> {
        fs::create_dir_all(&pathbuf)?;

        let env = unsafe { Env::new_nolock(Self::path_sanakirja_(&pathbuf), PAGE_SIZE, 1)? };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = txn
            .root_db(ROOT_DB)
            .unwrap_or_else(|| unsafe { btree::create_db_(&mut txn).unwrap() });

        Ok(Self { pathbuf, db, txn })
    }

    pub fn path_sanakirja(&self) -> PathBuf {
        Self::path_sanakirja_(&self.pathbuf)
    }
    fn path_sanakirja_(path: &Path) -> PathBuf {
        path.join("sanakirja")
    }

    pub fn path_self_defragmented(&self) -> PathBuf {
        let defragmented_path_opt: Option<Component> = self.pathbuf.components().last();
        let folder = match defragmented_path_opt {
            Some(Component::Normal(f)) => f.to_str().unwrap(),
            _ => unreachable!(),
        };
        let mut original_path = self.pathbuf.clone();
        original_path.pop();
        original_path.join(format!("{folder}-defragmented"))
    }

    pub fn read_length(&self) -> usize {
        Self::read_length_(&self.pathbuf)
    }
    pub fn read_length_(path: &Path) -> usize {
        fs::read(Self::path_length(path))
            .map(|v| {
                let mut buf = [0_u8; 8];
                v.iter().enumerate().take(8).for_each(|(i, b)| {
                    buf[i] = *b;
                });
                usize::from_le_bytes(buf)
            })
            .unwrap_or_default()
    }
    pub fn write_length(&self, len: usize) -> Result<(), io::Error> {
        Self::write_length_(&self.pathbuf, len)
    }
    pub fn write_length_(path: &Path, len: usize) -> Result<(), io::Error> {
        fs::write(Self::path_length(path), len.to_le_bytes())
    }
    fn path_length(path: &Path) -> PathBuf {
        path.join("length")
    }

    #[inline]
    pub fn get(&self, key: &Key) -> Result<Option<&Value>, Error> {
        let option = btree::get(&self.txn, &self.db, key, None)?;
        if let Some((key_found, v)) = option {
            if key == key_found {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    /// Iterate over key/value pairs from the database (disk)
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn iter(
        &self,
    ) -> Result<btree::Iter<'_, MutTxn<Env, ()>, Key, Value, page::Page<Key, Value>>, Error> {
        btree::iter(&self.txn, &self.db, None)
    }

    pub fn put(&mut self, key: &Key, value: &Value) -> Result<bool, Error> {
        btree::put(&mut self.txn, &mut self.db, key, value)
    }

    pub fn del(&mut self, key: &Key, value: Option<&Value>) -> Result<bool, Error> {
        btree::del(&mut self.txn, &mut self.db, key, value)
    }

    fn get_file_size_to_data_size_ratio(&self, len: usize) -> Result<f64, Error> {
        let data_bytes = (len * Self::KEY_AND_VALUE_SIZE) as f64;
        let file_bytes = File::open(&self.pathbuf)?.metadata()?.len() as f64;
        Ok(file_bytes / data_bytes)
    }

    pub fn should_defragment(&self, len: usize) -> Result<bool, Error> {
        Ok(self.get_file_size_to_data_size_ratio(len)? >= DEFRAGMENT_RATIO_THRESHOLD)
    }

    pub fn iter_collect(&self) -> Result<BTreeMap<&Key, &Value>, Error> {
        self.iter()?.collect::<_>()
    }

    pub fn iter_collect_multi(&self) -> Result<BTreeMap<&Key, Vec<&Value>>, Error> {
        let mut tree: BTreeMap<_, Vec<_>> = BTreeMap::new();
        self.iter()?.try_for_each(|res| -> Result<(), Error> {
            let (key, value): (&Key, &Value) = res?;
            tree.entry(key).or_default().push(value);
            Ok(())
        })?;
        Ok(tree)
    }

    pub fn commit(mut self, len: usize) -> Result<(), Error> {
        // dbg!(&self.pathbuf, len);
        // panic!();
        self.write_length(len)?;
        self.txn.set_root(ROOT_DB, self.db.db.into());
        self.txn.commit()
    }

    pub fn destroy(self) -> io::Result<()> {
        let path = self.pathbuf.to_owned();
        drop(self);
        fs::remove_dir_all(&path)
    }

    pub fn path(&self) -> &Path {
        &self.pathbuf
    }
}
