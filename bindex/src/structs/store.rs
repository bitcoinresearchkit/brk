use std::{
    array, fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use rayon::prelude::*;
use snkrj::{Database, DatabaseKey, DatabaseValue, UnitDatabase};
use storable_vec::UnsafeSizedSerDe;

use super::{Height, Version};

pub struct Store<K, V>
where
    K: DatabaseKey,
    V: DatabaseValue,
{
    pathbuf: PathBuf,
    version: Version,
    height: Option<Height>,
    len: usize,
    pub parts: [OnceLock<Box<Database<K, V>>>; 256],
}

impl<K, V> Store<K, V>
where
    K: DatabaseKey,
    V: DatabaseValue,
{
    pub fn open(path: &Path, version: Version) -> Result<Self, snkrj::Error> {
        fs::create_dir_all(path)?;

        let is_same_version =
            Version::try_from(Self::path_version_(path).as_path()).is_ok_and(|prev_version| version == prev_version);

        if !is_same_version {
            fs::remove_dir(path)?;
            fs::create_dir_all(path)?;
        }

        let height = Height::try_from(Self::path_height_(path).as_path()).ok();

        Ok(Self {
            pathbuf: path.to_owned(),
            version,
            height,
            len: UnitDatabase::read_length_(path),
            parts: array::from_fn(|_| OnceLock::new()),
        })
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.len
    }

    fn key_to_byte(key: &K) -> u8 {
        let slice = key.unsafe_as_slice();

        *(if cfg!(target_endian = "big") {
            slice.last()
        } else {
            slice.first()
        })
        .unwrap()
    }

    fn get_or_init_store(&self, key: &K) -> &Database<K, V> {
        self.get_or_init_store_(Self::key_to_byte(key) as usize)
    }

    fn get_or_init_store_(&self, storeindex: usize) -> &Database<K, V> {
        self.parts[storeindex]
            .get_or_init(|| Box::new(Database::open(self.path_parts().join(storeindex.to_string())).unwrap()))
    }

    fn get_or_init_mut_store(&mut self, key: &K) -> &mut Database<K, V> {
        self.get_or_init_store(key);

        self.parts
            .get_mut(Self::key_to_byte(key) as usize)
            .unwrap()
            .get_mut()
            .unwrap()
    }

    #[allow(unused)]
    pub fn open_all(&self) {
        (0..=(u8::MAX) as usize).for_each(|storeindex| {
            self.get_or_init_store_(storeindex);
        });
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_or_init_store(key).get(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.len += 1;
        self.get_or_init_mut_store(&key).insert(key, value)
    }

    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            self.insert(key, value);
        }
    }

    pub fn export(mut self, height: Height) -> Result<(), snkrj::Error> {
        if self.height.is_some_and(|self_height| self_height >= height) {
            return Ok(());
        }

        self.height = Some(height);
        self.version.write(&self.path_version())?;
        height.write(&self.path_height())?;
        UnitDatabase::write_length_(&self.pathbuf, self.len)?;
        self.parts.into_par_iter().try_for_each(|s| {
            if let Some(db) = s.into_inner() {
                db.export()
            } else {
                Ok(())
            }
        })
    }

    fn path_parts(&self) -> PathBuf {
        Self::path_parts_(&self.pathbuf)
    }
    fn path_parts_(path: &Path) -> PathBuf {
        path.join("parts")
    }

    fn path_version(&self) -> PathBuf {
        Self::path_version_(&self.pathbuf)
    }
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    pub fn height(&self) -> Option<&Height> {
        self.height.as_ref()
    }
    pub fn needs(&self, height: Height) -> bool {
        self.height.is_none_or(|self_height| height > self_height)
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(&self.pathbuf)
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }
}
