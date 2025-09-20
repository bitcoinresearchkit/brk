use std::{fs, io, path::Path};

use brk_interface::{Index, Interface};
use brk_server::VERSION;
use brk_structs::pools;

use crate::website::Website;

const BRIDGE_PATH: &str = "scripts/bridge";

#[allow(clippy::upper_case_acronyms)]
pub trait Bridge {
    fn generate_bridge_files(&self, website: Website, websites_path: &Path) -> io::Result<()>;
}

impl Bridge for Interface<'static> {
    fn generate_bridge_files(&self, website: Website, websites_path: &Path) -> io::Result<()> {
        if website.is_none() {
            return Ok(());
        }

        let path = websites_path.join(website.to_folder_name());

        if !fs::exists(&path)? {
            return Ok(());
        }

        let path = path.join(BRIDGE_PATH);

        fs::create_dir_all(&path)?;

        generate_vecs_file(self, &path)?;
        generate_pools_file(&path)
    }
}

fn generate_pools_file(parent: &Path) -> io::Result<()> {
    let path = parent.join(Path::new("pools.js"));

    let pools = pools();

    let mut contents = "//
// File auto-generated, any modifications will be overwritten
//
"
    .to_string();

    contents += "
/** @typedef {ReturnType<typeof createPools>} Pools */
/** @typedef {keyof Pools} Pool */

export function createPools() {
return /** @type {const} */ ({
";

    let mut sorted_pools: Vec<_> = pools.iter().collect();
    sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    contents += &sorted_pools
        .iter()
        .map(|pool| {
            let id = pool.serialized_id();
            format!("    {id}: \"{}\",", pool.name)
        })
        .collect::<Vec<_>>()
        .join("\n");

    contents += "\n  });\n}\n";

    fs::write(path, contents)
}

fn generate_vecs_file(interface: &Interface<'static>, parent: &Path) -> io::Result<()> {
    let path = parent.join(Path::new("vecs.js"));

    let indexes = Index::all();

    let mut contents = format!(
        "//
// File auto-generated, any modifications will be overwritten
//

export const VERSION = \"v{VERSION}\";

"
    );

    contents += &indexes
        .iter()
        .enumerate()
        .map(|(i_of_i, i)| {
            // let lowered = i.to_string().to_lowercase();
            format!("/** @typedef {{{i_of_i}}} {i} */",)
        })
        .collect::<Vec<_>>()
        .join("\n");

    contents += &format!(
        "\n\n/** @typedef {{{}}} Index */\n",
        indexes
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    );

    contents += "
/** @typedef {ReturnType<typeof createIndexes>} Indexes */

export function createIndexes() {
return {
";

    contents += &indexes
        .iter()
        .enumerate()
        .map(|(i_of_i, i)| {
            let lowered = i.to_string().to_lowercase();
            format!("    {lowered}: /** @satisfies {{{i}}} */ ({i_of_i}),",)
        })
        .collect::<Vec<_>>()
        .join("\n");

    contents += "  };\n}\n";

    contents += "
/** @typedef {ReturnType<typeof createVecIdToIndexes>} VecIdToIndexes
/** @typedef {keyof VecIdToIndexes} VecId */

/**
* @returns {Record<any, number[]>}
*/
export function createVecIdToIndexes() {
return {
";

    interface
        .id_to_index_to_vec()
        .iter()
        .for_each(|(id, index_to_vec)| {
            let indexes = index_to_vec
                .keys()
                .map(|i| (*i as u8).to_string())
                // .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            contents += &format!("    \"{id}\": [{indexes}],\n");
        });

    contents += "  };\n}\n";

    fs::write(path, contents)
}
