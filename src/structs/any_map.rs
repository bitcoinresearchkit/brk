use std::path::{Path, PathBuf};

use serde_json::Value;

use super::{MapKind, MapPath};

pub trait AnyMap {
    fn path(&self) -> &Path;
    fn path_last(&self) -> &Option<MapPath>;

    fn last_value(&self) -> Option<Value>;

    fn t_name(&self) -> &str;

    fn get_paths_to_type(&self) -> Vec<(PathBuf, String)> {
        let t_name = self.t_name().to_string();

        if let Some(path_last) = self.path_last() {
            vec![
                (self.path().to_owned(), t_name.clone()),
                (path_last.unwrap().to_owned(), t_name),
            ]
        } else {
            vec![(self.path().to_owned(), t_name)]
        }
    }

    fn pre_export(&mut self);
    fn export(&self) -> color_eyre::Result<()>;
    fn post_export(&mut self);

    fn delete_files(&self);

    fn kind(&self) -> MapKind;
}
