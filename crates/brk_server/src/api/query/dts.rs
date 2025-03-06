use std::{fs, io, path::Path};

use brk_query::{Index, Query};

use crate::Frontend;

const SCRIPTS: &str = "scripts";
const TPYES: &str = "types";

#[allow(clippy::upper_case_acronyms)]
pub trait DTS {
    fn generate_dts_file(&self, frontend: Frontend, websites_path: &Path) -> io::Result<()>;
}

impl DTS for Query<'static> {
    fn generate_dts_file(&self, frontend: Frontend, websites_path: &Path) -> io::Result<()> {
        if frontend.is_none() {
            return Ok(());
        }

        let path = websites_path.join(frontend.to_folder_name());

        if !fs::exists(&path)? {
            return Ok(());
        }

        let path = path.join(SCRIPTS).join(TPYES);

        fs::create_dir_all(&path)?;

        let path = path.join(Path::new("vecid-to-indexes.d.ts"));

        let mut contents = Index::all()
            .into_iter()
            .enumerate()
            .map(|(i_of_i, i)| format!("type {} = {};", i, i_of_i))
            .collect::<Vec<_>>()
            .join("\n");

        contents += "\n\ninterface VecIdToIndexes {\n";

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

        fs::write(path, contents)
    }
}
