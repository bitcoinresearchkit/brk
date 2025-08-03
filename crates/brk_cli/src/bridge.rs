use std::{fs, io, path::Path};

use brk_interface::{Index, Interface};
use brk_server::VERSION;

use crate::website::Website;

const SCRIPTS: &str = "scripts";

#[allow(clippy::upper_case_acronyms)]
pub trait Bridge {
    fn generate_bridge_file(&self, website: Website, websites_path: &Path) -> io::Result<()>;
}

impl Bridge for Interface<'static> {
    fn generate_bridge_file(&self, website: Website, websites_path: &Path) -> io::Result<()> {
        if website.is_none() {
            return Ok(());
        }

        let path = websites_path.join(website.to_folder_name());

        if !fs::exists(&path)? {
            return Ok(());
        }

        let path = path.join(SCRIPTS);

        fs::create_dir_all(&path)?;

        let path = path.join(Path::new("vecid-to-indexes.js"));

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

        self.id_to_index_to_vec()
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
}
