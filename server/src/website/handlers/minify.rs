// Simplified version of: https://github.com/swc-project/swc/blob/main/crates/swc/examples/minify.rs

use std::{path::Path, sync::Arc};

use swc::{config::JsMinifyOptions, try_with_handler, JsMinifyExtras};
use swc_common::{SourceMap, GLOBALS};

pub fn minify_js(path: &Path) -> String {
    let source_map = Arc::<SourceMap>::default();
    let compiler = swc::Compiler::new(source_map.clone());

    GLOBALS
        .set(&Default::default(), || {
            try_with_handler(source_map.clone(), Default::default(), |handler| {
                let fm = source_map.load_file(path).expect("failed to load file");

                compiler.minify(fm, handler, &JsMinifyOptions::default(), JsMinifyExtras::default())
            })
        })
        .unwrap()
        .code
}
