use std::{fs, io};

use brk_query::{Index, Query};

use crate::WEBSITE_DEV_PATH;

#[allow(clippy::upper_case_acronyms)]
pub trait DTS {
    fn generate_dts_file(&self) -> io::Result<()>;
}

impl DTS for Query<'static> {
    fn generate_dts_file(&self) -> io::Result<()> {
        if !fs::exists(WEBSITE_DEV_PATH)? {
            return Ok(());
        }

        let path = format!("{WEBSITE_DEV_PATH}/scripts/types/vecid-to-indexes.d.ts");

        let mut contents = Index::all()
            .into_iter()
            .enumerate()
            .map(|(i_of_i, i)| format!("type {} = {};", i, i_of_i))
            .collect::<Vec<_>>()
            .join("\n");

        contents += "\n\ninterface VecIdToIndexes {\n";

        self.vecid_to_index_to_vec.iter().for_each(|(id, index_to_vec)| {
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
