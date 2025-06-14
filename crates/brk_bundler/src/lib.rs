use std::{fs, io, path::Path, sync::Arc};

use brk_rolldown::{Bundler, BundlerOptions, RawMinifyOptions, SourceMapType};
use log::error;
use minify_html_onepass::Cfg;
use notify::{EventKind, RecursiveMode, Watcher};
use sugar_path::SugarPath;
use tokio::sync::Mutex;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn bundle(websites_path: &Path, source_folder: &str, watch: bool) -> io::Result<()> {
    let source_path = websites_path.join(source_folder);
    let dist_path = websites_path.join("dist");

    let _ = fs::remove_dir_all(&dist_path);
    copy_dir_all(&source_path, &dist_path)?;

    let source_scripts = format!("./{source_folder}/scripts");
    let source_entry = format!("{source_scripts}/entry.js");

    let absolute_websites_path = websites_path.absolutize();

    let mut bundler = Bundler::new(BundlerOptions {
        input: Some(vec![source_entry.into()]),
        dir: Some("./dist/scripts".to_string()),
        cwd: Some(absolute_websites_path),
        minify: Some(RawMinifyOptions::Bool(true)),
        sourcemap: Some(SourceMapType::File),
        ..Default::default()
    });

    bundler.write().await.unwrap();

    let absolute_source_index_path = source_path.join("index.html").absolutize();
    let absolute_source_index_path_clone = absolute_source_index_path.clone();
    let absolute_source_path = source_path.absolutize();
    let absolute_source_path_clone = absolute_source_path.clone();
    let absolute_source_scripts_path = websites_path.join(source_scripts).absolutize();
    let absolute_source_sw_path = source_path.join("service-worker.js").absolutize();
    let absolute_source_sw_path_clone = absolute_source_sw_path.clone();

    let absolute_dist_entry_path = dist_path.join("scripts/entry.js").absolutize();
    let absolute_dist_index_path = dist_path.join("index.html").absolutize();
    let absolute_dist_path = dist_path.absolutize();
    let absolute_dist_path_clone = absolute_dist_path.clone();
    let absolute_dist_sw_path = dist_path.join("service-worker.js").absolutize();

    let write_index = move || {
        let mut contents = fs::read_to_string(&absolute_source_index_path).unwrap();

        if let Ok(entry) = fs::read_to_string(absolute_dist_path_clone.join("scripts/entry.js")) {
            let start = entry.find("main").unwrap();
            let end = entry.find(".js").unwrap();
            let main_hashed = &entry[start..end];
            contents = contents.replace("/scripts/main.js", &format!("/scripts/{main_hashed}.js"));
        }

        if let Ok(contents) = minify_html_onepass::in_place_str(
            contents.as_mut_str(),
            &Cfg {
                minify_js: false,
                minify_css: false,
            },
        ) {
            let _ = fs::write(&absolute_dist_index_path, contents);
        }
    };

    let write_sw = move || {
        let contents = fs::read_to_string(&absolute_source_sw_path)
            .unwrap()
            .replace("__VERSION__", &format!("v{VERSION}"));
        let _ = fs::write(&absolute_dist_sw_path, contents);
    };

    write_index();
    write_sw();

    if !watch {
        return Ok(());
    }

    tokio::spawn(async move {
        let write_index_clone = write_index.clone();

        let mut entry_watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(_) => write_index_clone(),
                Err(e) => error!("watch error: {:?}", e),
            },
        )
        .unwrap();

        entry_watcher
            .watch(&absolute_dist_entry_path, RecursiveMode::Recursive)
            .unwrap();

        let mut source_watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) => event.paths,
                    EventKind::Modify(_) => event.paths,
                    _ => vec![],
                }
                .into_iter()
                .filter(|path| path.starts_with(&absolute_source_path))
                .filter(|path| !path.starts_with(&absolute_source_scripts_path))
                .for_each(|source_path| {
                    let suffix = source_path.strip_prefix(&absolute_source_path).unwrap();
                    let dist_path = absolute_dist_path.join(suffix);

                    if source_path == absolute_source_index_path_clone {
                        write_index();
                    } else if source_path == absolute_source_sw_path_clone {
                        write_sw();
                    } else {
                        let _ = fs::copy(&source_path, &dist_path);
                    }
                }),
                Err(e) => error!("watch error: {:?}", e),
            },
        )
        .unwrap();

        source_watcher
            .watch(&absolute_source_path_clone, RecursiveMode::Recursive)
            .unwrap();

        let watcher =
            brk_rolldown::Watcher::new(vec![Arc::new(Mutex::new(bundler))], None).unwrap();

        watcher.start().await;
    });

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
