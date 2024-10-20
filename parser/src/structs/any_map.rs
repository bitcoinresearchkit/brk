use std::path::{Path, PathBuf};

use serde_json::Value;

pub trait AnyMap {
    fn path(&self) -> &Path;
    fn path_last(&self) -> &Option<PathBuf>;

    fn last_value(&self) -> Option<Value>;

    fn t_name(&self) -> &str;

    fn exported_path_with_t_name(&self) -> Vec<(&Path, &str)> {
        let t_name = self.t_name();

        if let Some(path_last) = self.path_last() {
            vec![(self.path(), t_name), (path_last, t_name)]
        } else {
            vec![(self.path(), t_name)]
        }
    }

    fn pre_export(&mut self);
    fn export(&self) -> color_eyre::Result<()>;
    fn post_export(&mut self);

    fn delete_files(&self);
}
