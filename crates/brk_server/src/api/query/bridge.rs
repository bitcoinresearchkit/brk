use std::{fs, io, path::Path};

use brk_query::{Index, Query};

use crate::{VERSION, Website};

const SCRIPTS: &str = "scripts";

#[allow(clippy::upper_case_acronyms)]
pub trait Bridge {
    fn generate_bridge_file(&self, website: Website, websites_path: &Path) -> io::Result<()>;
}

impl Bridge for Query<'static> {
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

export const VERSION = \"v{}\";

",
            VERSION
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
            "\n\n/** @typedef {{{}}} Index */",
            indexes
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        );

        contents += "\n\n/** @typedef {ReturnType<typeof createVecIdToIndexes>} VecIdToIndexes */";
        contents += "\n/** @typedef {keyof VecIdToIndexes} VecId */\n";

        contents += "\nexport function createVecIdToIndexes() {\n";

        contents += "  return {\n";

        self.vec_trees
            .id_to_index_to_vec
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
