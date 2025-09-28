use std::{
    collections::{BTreeMap, HashMap},
    fs, io,
    path::Path,
};

use brk_interface::{Index, Interface};
use brk_structs::pools;

use super::VERSION;

const AUTO_GENERATED_DISCLAIMER: &str = "//
// File auto-generated, any modifications will be overwritten
//";

#[allow(clippy::upper_case_acronyms)]
pub trait Bridge {
    fn generate_js_files(&self, packages_path: &Path) -> io::Result<()>;
}

impl Bridge for Interface<'static> {
    fn generate_js_files(&self, packages_path: &Path) -> io::Result<()> {
        let path = packages_path.join("brk");

        if !fs::exists(&path)? {
            return Ok(());
        }

        let path = path.join("generated");
        fs::create_dir_all(&path)?;

        generate_version_file(&path)?;
        generate_metrics_file(self, &path)?;
        generate_pools_file(&path)
    }
}

fn generate_version_file(parent: &Path) -> io::Result<()> {
    let path = parent.join(Path::new("metrics.js"));

    let contents = format!(
        "{AUTO_GENERATED_DISCLAIMER}

export const VERSION = \"v{VERSION}\";
"
    );

    fs::write(path, contents)
}

fn generate_pools_file(parent: &Path) -> io::Result<()> {
    let path = parent.join(Path::new("pools.js"));

    let pools = pools();

    let mut contents = format!("{AUTO_GENERATED_DISCLAIMER}\n");

    contents += "
/**
 * @typedef {typeof POOL_ID_TO_POOL_NAME} PoolIdToPoolName
 * @typedef {keyof PoolIdToPoolName} PoolId
 */

export const POOL_ID_TO_POOL_NAME = /** @type {const} */ ({
";

    let mut sorted_pools: Vec<_> = pools.iter().collect();
    sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    contents += &sorted_pools
        .iter()
        .map(|pool| {
            let id = pool.serialized_id();
            format!("  {id}: \"{}\",", pool.name)
        })
        .collect::<Vec<_>>()
        .join("\n");

    contents += "\n});\n";

    fs::write(path, contents)
}

fn generate_metrics_file(interface: &Interface<'static>, parent: &Path) -> io::Result<()> {
    let path = parent.join(Path::new("metrics.js"));

    let indexes = Index::all();

    let mut contents = format!(
        "{AUTO_GENERATED_DISCLAIMER}

/**
"
    );

    contents += &indexes
        .iter()
        .map(|i| format!(" * @typedef {{\"{}\"}} {i}", i.serialize_long()))
        .collect::<Vec<_>>()
        .join("\n");

    contents += &format!(
        "
 * @typedef {{{}}} Index
 */

",
        indexes
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    );

    let mut unique_index_groups = BTreeMap::new();

    let mut word_to_freq: BTreeMap<_, usize> = BTreeMap::new();
    interface
        .metric_to_index_to_vec()
        .keys()
        .for_each(|metric| {
            metric.split("_").for_each(|word| {
                *word_to_freq.entry(word).or_default() += 1;
            });
        });
    let mut word_to_freq = word_to_freq.into_iter().collect::<Vec<_>>();
    word_to_freq.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let words = word_to_freq
        .into_iter()
        .map(|(str, _)| str)
        .collect::<Vec<_>>();

    contents += &format!(
        "export const INDEX_TO_WORD = [
  {}
];

",
        words
            .iter()
            .map(|w| format!("\"{w}\""))
            .collect::<Vec<_>>()
            .join(",\n  ")
    );

    let word_to_base62 = words
        .into_iter()
        .enumerate()
        .map(|(i, w)| (w, index_to_letters(i)))
        .collect::<HashMap<_, _>>();

    let mut ser_metric_to_indexes = "
/** @type {Record<string, Index[]>} */
export const COMPRESSED_METRIC_TO_INDEXES = {
"
    .to_string();

    interface
        .metric_to_index_to_vec()
        .iter()
        .for_each(|(metric, index_to_vec)| {
            let indexes = index_to_vec
                .keys()
                .map(|i| format!("\"{}\"", i.serialize_long()))
                .collect::<Vec<_>>()
                .join(", ");
            let indexes = format!("[{indexes}]");
            let unique = unique_index_groups.len();
            let index = index_to_letters(*unique_index_groups.entry(indexes).or_insert(unique));

            let compressed_metric = metric.split('_').fold(String::new(), |mut acc, w| {
                if !acc.is_empty() {
                    acc.push('_');
                }
                acc.push_str(&word_to_base62[w]);
                acc
            });

            ser_metric_to_indexes += &format!("  {compressed_metric}: {index},\n");
        });

    ser_metric_to_indexes += "};
";

    let mut sorted_groups: Vec<_> = unique_index_groups.into_iter().collect();
    sorted_groups.sort_by_key(|(_, index)| *index);
    sorted_groups.into_iter().for_each(|(group, index)| {
        let index = index_to_letters(index);
        contents += &format!("/** @type {{Index[]}} */\nconst {index} = {group};\n");
    });

    contents += &ser_metric_to_indexes;

    fs::write(path, contents)
}

fn index_to_letters(mut index: usize) -> String {
    if index < 52 {
        return (index_to_char(index) as char).to_string();
    }
    let mut result = [0u8; 8];
    let mut pos = 8;
    loop {
        pos -= 1;
        result[pos] = index_to_char(index % 52);
        index /= 52;
        if index == 0 {
            break;
        }
        index -= 1;
    }
    unsafe { String::from_utf8_unchecked(result[pos..].to_vec()) }
}

fn index_to_char(index: usize) -> u8 {
    match index {
        0..=25 => b'A' + index as u8,
        26..=51 => b'a' + (index - 26) as u8,
        _ => unreachable!(),
    }
}

// fn letters_to_index(s: &str) -> usize {
//     let mut result = 0;
//     for byte in s.bytes() {
//         let value = char_to_index(byte) as usize;
//         result = result * 52 + value + 1;
//     }
//     result - 1
// }

// fn char_to_index(byte: u8) -> u8 {
//     match byte {
//         b'A'..=b'Z' => byte - b'A',
//         b'a'..=b'z' => byte - b'a' + 26,
//         _ => 255, // Invalid
//     }
// }
