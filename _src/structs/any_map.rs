use std::path::Path;

use serde_json::Value;

use crate::io::Serialization;

use super::{Config, MapKind, MapPath};

pub trait AnyMap {
    fn path(&self) -> &Path;
    fn path_parent(&self) -> &Path;
    fn path_last(&self) -> &Option<MapPath>;
    fn last_value(&self) -> Option<Value>;
    fn serialization(&self) -> Serialization;
    fn type_name(&self) -> &str;
    fn key_name(&self) -> &str;
    fn pre_export(&mut self);
    fn export(&self) -> color_eyre::Result<()>;
    fn post_export(&mut self);
    fn delete_files(&self);
    fn kind(&self) -> MapKind;
    fn id(&self, config: &Config) -> String;
}
