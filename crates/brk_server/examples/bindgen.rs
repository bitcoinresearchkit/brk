use std::{env, fs, path::PathBuf};

use aide::axum::ApiRouter;
use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::Vecs;
use brk_server::{ApiRoutes, finish_openapi, generate_bindings};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let tmp = env::temp_dir().join("brk_bindgen");
    fs::create_dir_all(&tmp)?;

    let indexer = Indexer::forced_import(&tmp)?;
    let computer = Computer::forced_import(&tmp, &indexer)?;
    let vecs = Vecs::build_rw(&indexer, &computer);

    let (_, openapi) = finish_openapi(ApiRouter::new().add_api_routes());

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .unwrap()
        .to_path_buf();

    let output_paths = brk_bindgen::ClientOutputPaths::new()
        .rust(workspace_root.join("crates/brk_client/src/lib.rs"))
        .javascript(workspace_root.join("website/scripts/modules/brk-client/index.js"))
        .python(workspace_root.join("packages/brk_client/brk_client/__init__.py"));

    generate_bindings(&vecs, &openapi, &output_paths)?;

    fs::remove_dir_all(&tmp)?;

    eprintln!("Done");

    Ok(())
}
