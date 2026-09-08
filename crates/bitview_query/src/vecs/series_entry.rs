use bitview_plugin::Plugin;
use brk_types::Index;
use vecdb::AnyExportableVec;

/// A queryable vector together with the plugin that owns it.
#[derive(Clone, Copy)]
pub struct SeriesEntry<'a> {
    vec: &'a dyn AnyExportableVec,
    plugin: &'a dyn Plugin,
    index: Index,
    is_mutable: bool,
}

pub enum SeriesEntryLookup<'a> {
    Found(SeriesEntry<'a>),
    Unsupported(Vec<Index>),
    Missing,
}

impl<'a> SeriesEntry<'a> {
    pub fn new(
        index: Index,
        vec: &'a dyn AnyExportableVec,
        plugin: &'a dyn Plugin,
        is_mutable: bool,
    ) -> Self {
        Self {
            vec,
            plugin,
            index,
            is_mutable,
        }
    }

    pub fn index(self) -> Index {
        self.index
    }

    pub fn vec(self) -> &'a dyn AnyExportableVec {
        self.vec
    }

    pub fn plugin(self) -> &'a dyn Plugin {
        self.plugin
    }

    /// Whether existing values may change, preventing an immutable cache prefix.
    /// Publication guards are required independently of this flag.
    pub fn is_mutable(self) -> bool {
        self.is_mutable
    }
}
