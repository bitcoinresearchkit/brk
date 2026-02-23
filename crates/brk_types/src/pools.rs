use std::sync::OnceLock;

use serde::Deserialize;

use crate::PoolSlug;

use super::Pool;

const JSON_DATA: &str = include_str!("../pools-v2.json");
const POOL_COUNT: usize = 168;
const TESTNET_IDS: &[u16] = &[145, 146, 149, 150, 156, 163];

#[derive(Deserialize)]
struct JsonPoolEntry {
    id: u16,
    name: String,
    addresses: Vec<String>,
    tags: Vec<String>,
    link: String,
}

fn leak_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn empty_pool(id: usize) -> Pool {
    Pool {
        slug: PoolSlug::from(id as u8),
        name: "",
        addresses: Box::new([]),
        tags: Box::new([]),
        tags_lowercase: Box::new([]),
        link: "",
    }
}

#[derive(Debug)]
pub struct Pools(Vec<Pool>);

impl Pools {
    pub fn find_from_coinbase_tag(&self, coinbase_tag: &str) -> Option<&Pool> {
        let coinbase_tag = coinbase_tag.to_lowercase();
        self.iter().find(|pool| {
            pool.tags_lowercase
                .iter()
                .any(|pool_tag| coinbase_tag.contains(pool_tag))
        })
    }

    pub fn find_from_address(&self, address: &str) -> Option<&Pool> {
        self.iter().find(|pool| pool.addresses.contains(&address))
    }

    pub fn get_unknown(&self) -> &Pool {
        &self.0[0]
    }

    pub fn get(&self, slug: PoolSlug) -> &Pool {
        let i: u8 = slug.into();
        &self.0[i as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pool> + '_ {
        self.0.iter().filter(|p| !p.name.is_empty())
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        POOL_COUNT - TESTNET_IDS.len()
    }
}

pub fn pools() -> &'static Pools {
    static POOLS: OnceLock<Pools> = OnceLock::new();
    POOLS.get_or_init(|| {
        let entries: Vec<JsonPoolEntry> =
            serde_json::from_str(JSON_DATA).expect("Failed to parse pools-v2.json");

        let mut pools: Vec<Pool> = (0..POOL_COUNT).map(empty_pool).collect();

        // Position 0: Unknown pool
        pools[0] = Pool {
            slug: PoolSlug::Unknown,
            name: "Unknown",
            addresses: Box::new([]),
            tags: Box::new([]),
            tags_lowercase: Box::new([]),
            link: "",
        };

        for entry in entries {
            if TESTNET_IDS.contains(&entry.id) {
                continue;
            }
            let id = entry.id as usize;
            let slug = PoolSlug::from(id as u8);
            let tags_lowercase = entry
                .tags
                .iter()
                .map(|t| t.to_lowercase())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            pools[id] = Pool {
                slug,
                name: leak_str(entry.name),
                link: leak_str(entry.link),
                addresses: entry
                    .addresses
                    .into_iter()
                    .map(leak_str)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                tags: entry
                    .tags
                    .into_iter()
                    .map(leak_str)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                tags_lowercase,
            };
        }

        Pools(pools)
    })
}
