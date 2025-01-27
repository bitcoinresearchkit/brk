use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use canopydb::{Tree as CanopyTree, TreeOptions, WriteTransaction};
use color_eyre::eyre::eyre;

#[derive(Debug)]
pub struct Tree<'a, K, V> {
    tree: CanopyTree<'a>,
    k: PhantomData<K>,
    v: PhantomData<V>,
}
impl<'a, K, V> Deref for Tree<'a, K, V> {
    type Target = CanopyTree<'a>;
    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}
impl<'a, K, V> DerefMut for Tree<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

impl<'a, K, V> Tree<'a, K, V>
where
    K: Debug + Sized,
    V: Debug + Sized + Clone + Copy,
{
    const SIZE_OF_K: usize = size_of::<K>();
    const SIZE_OF_V: usize = size_of::<V>();

    pub fn new(tx: &'a WriteTransaction) -> color_eyre::Result<Self> {
        let mut options = TreeOptions::new();
        options.compress_overflow_values = None;
        options.fixed_key_len = size_of::<K>() as i8;
        options.fixed_value_len = size_of::<V>() as i8;

        Ok(Self {
            tree: tx.get_or_create_tree_with(b"tree", options)?,
            k: PhantomData,
            v: PhantomData,
        })
    }

    pub fn get(&self, key: &K) -> color_eyre::Result<Option<V>> {
        let slice = self.tree.get(Self::key_as_slice(key))?;

        if slice.is_none() {
            return Ok(None);
        }

        let slice = slice.unwrap();

        let (prefix, shorts, suffix) = unsafe { slice.align_to::<V>() };

        if !prefix.is_empty() || shorts.len() != 1 || !suffix.is_empty() {
            dbg!(&key, &prefix, &shorts, &suffix);
            return Err(eyre!("align_to issue"));
        }

        Ok(Some(shorts[0]))
    }

    pub fn insert(&mut self, key: &K, value: &V) -> Result<(), canopydb::Error> {
        self.tree
            .insert(Self::key_as_slice(key), Self::value_as_slice(value))
    }

    fn key_as_slice(key: &K) -> &[u8] {
        let data: *const K = key;
        let data: *const u8 = data as *const u8;
        unsafe { std::slice::from_raw_parts(data, Self::SIZE_OF_K) }
    }

    fn value_as_slice(value: &V) -> &[u8] {
        let data: *const V = value;
        let data: *const u8 = data as *const u8;
        unsafe { std::slice::from_raw_parts(data, Self::SIZE_OF_V) }
    }
}
