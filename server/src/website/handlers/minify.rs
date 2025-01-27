// Simplified version of: https://github.com/swc-project/swc/blob/main/crates/swc/examples/minify.rs

use std::{path::Path, sync::Arc};

use swc::{config::JsMinifyOptions, try_with_handler, JsMinifyExtras};
use swc_common::{SourceMap, GLOBALS};

pub fn minify_js(path: &Path) -> String {
    let cm = Arc::<SourceMap>::default();

    let c = swc::Compiler::new(cm.clone());

    let output = GLOBALS
        .set(&Default::default(), || {
            try_with_handler(cm.clone(), Default::default(), |handler| {
                let fm = cm.load_file(path).expect("failed to load file");

                c.minify(
                    fm,
                    handler,
                    &JsMinifyOptions::default(),
                    JsMinifyExtras::default(),
                )
            })
        })
        .unwrap();

    output.code
}
