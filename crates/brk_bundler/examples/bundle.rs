use std::{io, path::PathBuf, thread, time::Duration};

use brk_bundler::bundle;

fn find_dev_dirs() -> Option<(PathBuf, PathBuf)> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let websites = dir.join("websites");
        let modules = dir.join("modules");
        if websites.exists() && modules.exists() {
            return Some((websites, modules));
        }
        // Stop at workspace root (crates/ indicates we're there)
        if dir.join("crates").exists() {
            return None;
        }
        dir = dir.parent()?.to_path_buf();
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let (websites_path, modules_path) =
        find_dev_dirs().expect("Run from within the brk workspace");
    let source_folder = "bitview";

    let dist_path = bundle(&modules_path, &websites_path, source_folder, true).await?;

    println!("Bundle created at: {}", dist_path.display());
    println!("Watching for changes... (Ctrl+C to stop)");

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
