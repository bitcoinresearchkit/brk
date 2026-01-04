#![doc = include_str!("../README.md")]

use std::{
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

use log::{debug, error, info};
use notify::{EventKind, PollWatcher, RecursiveMode, Watcher};
use rolldown::{
    Bundler, BundlerConfig, BundlerOptions, InlineConstConfig, InlineConstMode, InlineConstOption,
    OptimizationOption, RawMinifyOptions, SourceMapType,
};
use sugar_path::SugarPath;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn bundle(
    modules_path: &Path,
    websites_path: &Path,
    source_folder: &str,
    watch: bool,
) -> io::Result<PathBuf> {
    let relative_modules_path = modules_path;
    let relative_source_path = websites_path.join(source_folder);
    let relative_dist_path = websites_path.join("dist");

    let absolute_modules_path = relative_modules_path.absolutize();
    let absolute_modules_path_clone = absolute_modules_path.clone();
    let absolute_websites_path = websites_path.absolutize();
    let absolute_websites_path_clone = absolute_websites_path.clone();

    let absolute_source_path = relative_source_path.absolutize();
    let absolute_source_index_path = absolute_source_path.join("index.html");
    let absolute_source_index_path_clone = absolute_source_index_path.clone();
    let absolute_source_scripts_path = absolute_source_path.join("scripts");
    let absolute_source_scripts_modules_path = absolute_source_scripts_path.join("modules");
    let absolute_source_sw_path = absolute_source_path.join("service-worker.js");
    let absolute_source_sw_path_clone = absolute_source_sw_path.clone();

    let absolute_dist_path = relative_dist_path.absolutize();
    let absolute_dist_scripts_path = absolute_dist_path.join("scripts");
    let absolute_dist_scripts_entry_path = absolute_dist_scripts_path.join("entry.js");
    let absolute_dist_scripts_entry_path_clone = absolute_dist_scripts_entry_path.clone();
    let absolute_dist_index_path = absolute_dist_path.join("index.html");
    let absolute_dist_sw_path = absolute_dist_path.join("service-worker.js");

    info!("Bundling {source_folder}...");
    info!("  modules: {absolute_modules_path:?}");
    info!("  source:  {absolute_source_path:?}");
    info!("  dist:    {absolute_dist_path:?}");

    let _ = fs::remove_dir_all(&absolute_dist_path);
    let _ = fs::remove_dir_all(&absolute_source_scripts_modules_path);
    copy_dir_all(
        &absolute_modules_path,
        &absolute_source_scripts_modules_path,
    )?;
    copy_dir_all(&absolute_source_path, &absolute_dist_path)?;
    fs::remove_dir_all(&absolute_dist_scripts_path)?;
    fs::create_dir(&absolute_dist_scripts_path)?;

    // dbg!(BundlerOptions::default());

    let bundler_options = BundlerOptions {
        input: Some(vec![format!("./{source_folder}/scripts/entry.js").into()]),
        dir: Some("./dist/scripts".to_string()),
        cwd: Some(absolute_websites_path),
        minify: Some(RawMinifyOptions::Bool(true)),
        sourcemap: Some(SourceMapType::File),
        // advanced_chunks: Some(AdvancedChunksOptions {
        //     // min_size: Some(1000.0),
        //     min_share_count: Some(20),
        //     // min_module_size: S
        //     // include_dependencies_recursively: Some(true),
        //     ..Default::default()
        // }),
        //
        // inline_dynamic_imports
        // experimental: Some(ExperimentalOptions {
        //     strict_execution_order: Some(true),
        //     ..Default::default()
        // }),
        optimization: Some(OptimizationOption {
            inline_const: Some(InlineConstOption::Config(InlineConstConfig {
                mode: Some(InlineConstMode::All),
                ..Default::default()
            })),
            // Needs benchmarks
            // pife_for_module_wrappers: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut bundler = Bundler::new(bundler_options.clone()).unwrap();

    if let Err(error) = bundler.write().await {
        error!("{error:?}");
    }

    let update_dist_index = move || {
        let mut contents = fs::read_to_string(&absolute_source_index_path).unwrap();

        if let Ok(entry) = fs::read_to_string(&absolute_dist_scripts_entry_path_clone)
            && let Some(start) = entry.find("main")
            && let Some(end) = entry.find(".js")
        {
            let main_hashed = &entry[start..end];
            contents = contents.replace("/scripts/main.js", &format!("/scripts/{main_hashed}.js"));
        }

        let _ = fs::write(&absolute_dist_index_path, contents);
    };

    let update_source_sw = move || {
        let contents = fs::read_to_string(&absolute_source_sw_path)
            .unwrap()
            .replace("__VERSION__", &format!("v{VERSION}"));
        let _ = fs::write(&absolute_dist_sw_path, contents);
    };

    update_dist_index();
    update_source_sw();

    if !watch {
        return Ok(relative_dist_path);
    }

    // Clone paths for the second watcher
    let absolute_websites_path_clone2 = absolute_websites_path_clone.clone();
    let absolute_modules_path_clone2 = absolute_modules_path_clone.clone();

    tokio::spawn(async move {
        let handle_event = {
            let absolute_dist_scripts_entry_path = absolute_dist_scripts_entry_path.clone();
            let absolute_source_index_path_clone = absolute_source_index_path_clone.clone();
            let absolute_source_sw_path_clone = absolute_source_sw_path_clone.clone();
            let absolute_modules_path = absolute_modules_path.clone();
            let absolute_source_scripts_modules_path = absolute_source_scripts_modules_path.clone();
            let absolute_source_path = absolute_source_path.clone();
            let absolute_source_scripts_path = absolute_source_scripts_path.clone();
            let absolute_dist_path = absolute_dist_path.clone();
            let update_dist_index = update_dist_index.clone();
            let update_source_sw = update_source_sw.clone();

            move |path: PathBuf| {
                let path = path.absolutize();

                if path == absolute_dist_scripts_entry_path
                    || path == absolute_source_index_path_clone
                {
                    update_dist_index();
                } else if path == absolute_source_sw_path_clone {
                    update_source_sw();
                } else if let Ok(suffix) = path.strip_prefix(&absolute_modules_path) {
                    let dest = absolute_source_scripts_modules_path.join(suffix);
                    if path.is_file() {
                        debug!("Copying module: {path:?} -> {dest:?}");
                        let _ = fs::create_dir_all(dest.parent().unwrap());
                        if let Err(e) = fs::copy(&path, &dest) {
                            error!("Copy failed: {e}");
                        }
                    }
                } else if let Ok(suffix) = path.strip_prefix(&absolute_source_path)
                    // scripts are handled by rolldown
                    && !path.starts_with(&absolute_source_scripts_path)
                {
                    let dist_path = absolute_dist_path.join(suffix);
                    if path.is_file() {
                        let _ = fs::create_dir_all(path.parent().unwrap());
                        let _ = fs::copy(&path, &dist_path);
                    }
                }
            }
        };

        // FSEvents watcher for instant response to manual saves
        let handle_event_clone = handle_event.clone();
        let mut fs_watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        event.paths.into_iter().for_each(&handle_event_clone);
                    }
                    _ => {}
                },
                Err(e) => error!("fs watch error: {e:?}"),
            },
        )
        .unwrap();

        fs_watcher
            .watch(&absolute_websites_path_clone, RecursiveMode::Recursive)
            .unwrap();
        fs_watcher
            .watch(&absolute_modules_path_clone, RecursiveMode::Recursive)
            .unwrap();

        // Poll watcher to catch programmatic edits (e.g., Claude Code's atomic writes)
        let poll_config = notify::Config::default()
            .with_poll_interval(Duration::from_secs(1));
        let mut poll_watcher = PollWatcher::new(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        event.paths.into_iter().for_each(&handle_event);
                    }
                    _ => {}
                },
                Err(e) => error!("poll watch error: {e:?}"),
            },
            poll_config,
        )
        .unwrap();

        poll_watcher
            .watch(&absolute_websites_path_clone2, RecursiveMode::Recursive)
            .unwrap();
        poll_watcher
            .watch(&absolute_modules_path_clone2, RecursiveMode::Recursive)
            .unwrap();

        let config = BundlerConfig::new(bundler_options, vec![]);
        let watcher = rolldown::Watcher::new(config, None).unwrap();

        watcher.start().await;
    });

    Ok(relative_dist_path)
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
