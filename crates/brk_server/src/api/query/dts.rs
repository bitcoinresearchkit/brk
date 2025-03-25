use std::{fs, io, path::Path};

use brk_query::{Index, Query};

use crate::Website;

const SCRIPTS: &str = "scripts";
const TPYES: &str = "types";

#[allow(clippy::upper_case_acronyms)]
pub trait DTS {
    fn generate_dts_file(&self, website: Website, websites_path: &Path) -> io::Result<()>;
}

impl DTS for Query<'static> {
    fn generate_dts_file(&self, website: Website, websites_path: &Path) -> io::Result<()> {
        if website.is_none() {
            return Ok(());
        }

        let path = websites_path.join(website.to_folder_name());

        if !fs::exists(&path)? {
            return Ok(());
        }

        let path = path.join(SCRIPTS).join(TPYES);

        fs::create_dir_all(&path)?;

        let path = path.join(Path::new("vecid-to-indexes.d.ts"));

        let indexes = Index::all();

        let mut contents = indexes
            .iter()
            .enumerate()
            .map(|(i_of_i, i)| format!("type {} = {};", i, i_of_i))
            .collect::<Vec<_>>()
            .join("\n");

        contents += &format!(
            "\n\nexport type Index = {};",
            indexes
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        );

        contents += "\n\nexport interface VecIdToIndexes {\n";

        self.vecid_to_index_to_vec
            .iter()
            .for_each(|(id, index_to_vec)| {
                let indexes = index_to_vec
                    .keys()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                contents += &format!(
                    "  {}: [{indexes}]\n",
                    if id.contains("-") {
                        format!("\"{id}\"")
                    } else {
                        id.to_owned()
                    }
                );
            });

        contents.push('}');

        contents += "\n\nexport type VecId = keyof VecIdToIndexes;";

        fs::write(path, contents)
    }
}
