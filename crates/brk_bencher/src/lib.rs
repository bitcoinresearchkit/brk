use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use brk_error::Result;

mod disk;
mod memory;
mod progression;

use disk::*;
use memory::*;
use parking_lot::Mutex;
use progression::*;

#[derive(Clone)]
pub struct Bencher(Arc<BencherInner>);

struct BencherInner {
    bench_dir: PathBuf,
    monitored_path: PathBuf,
    stop_flag: Arc<AtomicBool>,
    monitor_thread: Mutex<Option<JoinHandle<Result<()>>>>,
    progression: Arc<ProgressionMonitor>,
}

impl Bencher {
    /// Create a new bencher for the given crate name
    /// Creates directory structure: workspace_root/benches/{crate_name}/{timestamp}/
    pub fn new(crate_name: &str, workspace_root: &Path, monitored_path: &Path) -> Result<Self> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let bench_dir = workspace_root
            .join("benches")
            .join(crate_name)
            .join(timestamp.to_string());

        fs::create_dir_all(&bench_dir)?;

        let progress_csv = bench_dir.join("progress.csv");
        let progression = Arc::new(ProgressionMonitor::new(&progress_csv)?);
        let progression_clone = progression.clone();

        // Register hook with logger
        brk_logger::register_hook(move |message| {
            progression_clone.check_and_record(message);
        })
        .map_err(|e| io::Error::new(io::ErrorKind::AlreadyExists, e))?;

        Ok(Self(Arc::new(BencherInner {
            bench_dir,
            monitored_path: monitored_path.to_path_buf(),
            stop_flag: Arc::new(AtomicBool::new(false)),
            progression,
            monitor_thread: Mutex::new(None),
        })))
    }

    /// Create a bencher using CARGO_MANIFEST_DIR to find workspace root
    pub fn from_cargo_env(crate_name: &str, monitored_path: &Path) -> Result<Self> {
        let mut current = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))
            .unwrap();

        let workspace_root = loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists() {
                let contents = std::fs::read_to_string(&cargo_toml)
                    .map_err(|e| format!("Failed to read Cargo.toml: {}", e))
                    .unwrap();
                if contents.contains("[workspace]") {
                    break current;
                }
            }

            current = current
                .parent()
                .ok_or("Workspace root not found")?
                .to_path_buf();
        };

        Self::new(crate_name, &workspace_root, monitored_path)
    }

    /// Start monitoring disk usage and memory footprint
    pub fn start(&mut self) -> Result<()> {
        if self.0.monitor_thread.lock().is_some() {
            return Err("Bencher already started".into());
        }

        let stop_flag = self.0.stop_flag.clone();
        let bench_dir = self.0.bench_dir.clone();
        let monitored_path = self.0.monitored_path.clone();

        let handle =
            thread::spawn(move || monitor_resources(&monitored_path, &bench_dir, stop_flag));

        *self.0.monitor_thread.lock() = Some(handle);
        Ok(())
    }

    /// Stop monitoring and wait for the thread to finish
    pub fn stop(&self) -> Result<()> {
        self.0.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.0.monitor_thread.lock().take() {
            handle.join().map_err(|_| "Monitor thread panicked")??;
        }

        self.0.progression.flush()?;

        Ok(())
    }
}

impl Drop for Bencher {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn monitor_resources(
    monitored_path: &Path,
    bench_dir: &Path,
    stop_flag: Arc<AtomicBool>,
) -> Result<()> {
    let disk_file = bench_dir.join("disk.csv");
    let memory_file = bench_dir.join("memory.csv");

    let mut disk_writer = fs::File::create(disk_file)?;
    let mut memory_writer = fs::File::create(memory_file)?;

    writeln!(disk_writer, "timestamp_ms,disk_usage")?;
    writeln!(
        memory_writer,
        "timestamp_ms,phys_footprint,phys_footprint_peak"
    )?;

    let pid = std::process::id();
    let start = Instant::now();

    let mut disk_monitor = DiskMonitor::new();
    let memory_monitor = MemoryMonitor::new(pid);

    'l: loop {
        let elapsed_ms = start.elapsed().as_millis();

        if let Ok(bytes) = disk_monitor.get_disk_usage(monitored_path) {
            writeln!(disk_writer, "{},{}", elapsed_ms, bytes)?;
        }

        if let Ok((footprint, peak)) = memory_monitor.get_memory_usage() {
            writeln!(memory_writer, "{},{},{}", elapsed_ms, footprint, peak)?;
        }

        // Best version
        for _ in 0..50 {
            // 50 * 100ms = 5 seconds
            if stop_flag.load(Ordering::Relaxed) {
                break 'l;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
