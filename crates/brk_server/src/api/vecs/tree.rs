use std::{collections::BTreeMap, fs, io};

use brk_vec::AnyJsonStorableVec;
use derive_deref::{Deref, DerefMut};

use crate::WEBSITE_DEV_PATH;

use super::index::Index;

#[derive(Default, Deref, DerefMut)]
pub struct VecIdToIndexToVec(BTreeMap<String, IndexToVec>);

impl VecIdToIndexToVec {
    // Not the most performant or type safe but only built once so that's okay
    pub fn insert(&mut self, vec: &'static dyn AnyJsonStorableVec) {
        let file_name = vec.file_name();
        let split = file_name.split("_to_").collect::<Vec<_>>();
        if split.len() != 2 {
            panic!();
        }
        let str = vec.index_type_to_string().split("::").last().unwrap().to_lowercase();
        let index = Index::try_from(str.as_str())
            .inspect_err(|_| {
                dbg!(str);
            })
            .unwrap();
        if split[0] != index.to_string().to_lowercase() {
            dbg!(split[0], index.to_string());
            panic!();
        }
        let key = split[1].to_string().replace("_", "-");
        let prev = self.entry(key).or_default().insert(index, vec);
        if prev.is_some() {
            panic!()
        }
    }

    pub fn generate_dts_file(&self) -> io::Result<()> {
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

        self.iter().for_each(|(id, index_to_vec)| {
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

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec(BTreeMap<Index, &'static dyn AnyJsonStorableVec>);
