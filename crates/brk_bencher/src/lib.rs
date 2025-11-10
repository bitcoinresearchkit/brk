use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use brk_error::Result;

pub struct Bencher {
    bench_dir: PathBuf,
    stop_flag: Arc<AtomicBool>,
    monitor_thread: Option<JoinHandle<Result<()>>>,
}

impl Bencher {
    /// Create a new bencher for the given crate name
    /// Creates directory structure: workspace_root/benches/{crate_name}/{timestamp}/
    pub fn new(crate_name: &str, workspace_root: &Path) -> Result<Self> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let bench_dir = workspace_root
            .join("benches")
            .join(crate_name)
            .join(timestamp.to_string());

        fs::create_dir_all(&bench_dir)?;

        Ok(Self {
            bench_dir,
            stop_flag: Arc::new(AtomicBool::new(false)),
            monitor_thread: None,
        })
    }

    /// Create a bencher using CARGO_MANIFEST_DIR to find workspace root
    pub fn from_cargo_env() -> Result<Self> {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("Failed to find workspace root")?;
        let crate_name = env!("CARGO_PKG_NAME");
        Self::new(crate_name, workspace_root)
    }

    /// Start monitoring disk usage and memory footprint
    pub fn start(&mut self) -> Result<()> {
        if self.monitor_thread.is_some() {
            return Err("Bencher already started".into());
        }

        let stop_flag = self.stop_flag.clone();
        let bench_dir = self.bench_dir.clone();

        let handle = thread::spawn(move || monitor_resources(&bench_dir, stop_flag));

        self.monitor_thread = Some(handle);
        Ok(())
    }

    /// Stop monitoring and wait for the thread to finish
    pub fn stop(mut self) -> Result<()> {
        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.monitor_thread.take() {
            handle.join().map_err(|_| "Monitor thread panicked")??;
        }

        Ok(())
    }

    /// Get the benchmark output directory
    pub fn bench_dir(&self) -> &Path {
        &self.bench_dir
    }
}

fn parse_size_to_mb(value_str: &str, unit: &str) -> Option<f64> {
    let value: f64 = value_str.parse().ok()?;
    match unit {
        "MB" | "M" => Some(value),
        "GB" | "G" => Some(value * 1024.0),
        "KB" | "K" => Some(value / 1024.0),
        "B" => Some(value / 1024.0 / 1024.0),
        _ => None,
    }
}

fn parse_du_output(size_str: &str) -> Option<f64> {
    // Parse outputs like "524M", "287G", "4.0K"
    let size_str = size_str.trim();

    if let Some(unit_pos) = size_str.find(|c: char| c.is_alphabetic()) {
        let (value_part, unit_part) = size_str.split_at(unit_pos);
        parse_size_to_mb(value_part, unit_part)
    } else {
        // No unit means bytes
        let value: f64 = size_str.parse().ok()?;
        Some(value / 1024.0 / 1024.0)
    }
}

fn parse_footprint_output(output: &str) -> Option<(f64, f64)> {
    let mut phys_footprint = None;
    let mut phys_footprint_peak = None;

    for line in output.lines() {
        if line.contains("phys_footprint:") && !line.contains("peak") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                phys_footprint = parse_size_to_mb(parts[1], parts[2]);
            }
        } else if line.contains("phys_footprint_peak:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                phys_footprint_peak = parse_size_to_mb(parts[1], parts[2]);
            }
        }
    }

    match (phys_footprint, phys_footprint_peak) {
        (Some(f), Some(p)) => Some((f, p)),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn get_memory_usage_linux(pid: u32) -> Result<(f64, f64)> {
    // Read /proc/[pid]/status for memory information
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(status_path)?;

    let mut vm_rss = None;
    let mut vm_hwm = None;

    for line in status_content.lines() {
        if line.starts_with("VmRSS:") {
            // Current RSS in kB
            if let Some(value_str) = line.split_whitespace().nth(1)
                && let Ok(kb) = value_str.parse::<f64>()
            {
                vm_rss = Some(kb / 1024.0); // Convert kB to MB
            }
        } else if line.starts_with("VmHWM:") {
            // Peak RSS (High Water Mark) in kB
            if let Some(value_str) = line.split_whitespace().nth(1)
                && let Ok(kb) = value_str.parse::<f64>()
            {
                vm_hwm = Some(kb / 1024.0); // Convert kB to MB
            }
        }
    }

    match (vm_rss, vm_hwm) {
        (Some(rss), Some(hwm)) => Ok((rss, hwm)),
        _ => Err("Failed to parse memory info from /proc/[pid]/status".into()),
    }
}

#[cfg(target_os = "macos")]
fn get_memory_usage_macos(pid: u32) -> Result<(f64, f64)> {
    let output = Command::new("footprint")
        .args(["-p", &pid.to_string()])
        .output()?;

    let stdout = String::from_utf8(output.stdout).unwrap();
    parse_footprint_output(&stdout).ok_or_else(|| "Failed to parse footprint output".into())
}

fn get_memory_usage(pid: u32) -> Result<(f64, f64)> {
    #[cfg(target_os = "macos")]
    {
        get_memory_usage_macos(pid)
    }

    #[cfg(target_os = "linux")]
    {
        get_memory_usage_linux(pid)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported platform for memory monitoring".into())
    }
}

fn monitor_resources(bench_dir: &Path, stop_flag: Arc<AtomicBool>) -> Result<()> {
    let disk_file = bench_dir.join("disk_usage.csv");
    let memory_file = bench_dir.join("memory_footprint.csv");

    let mut disk_writer = fs::File::create(disk_file)?;
    let mut memory_writer = fs::File::create(memory_file)?;

    writeln!(disk_writer, "timestamp_ms,disk_usage_mb")?;
    writeln!(
        memory_writer,
        "timestamp_ms,phys_footprint_mb,phys_footprint_peak_mb"
    )?;

    let pid = std::process::id();
    let start = Instant::now();

    while !stop_flag.load(Ordering::Relaxed) {
        let elapsed_ms = start.elapsed().as_millis();

        // Get disk usage
        if let Ok(output) = Command::new("du")
            .args(["-sh", bench_dir.to_str().unwrap()])
            .output()
            && let Ok(stdout) = String::from_utf8(output.stdout)
            && let Some(size_str) = stdout.split_whitespace().next()
            && let Some(size_mb) = parse_du_output(size_str)
        {
            writeln!(disk_writer, "{},{}", elapsed_ms, size_mb)?;
            disk_writer.flush()?;
        }

        // Get memory footprint (cross-platform)
        if let Ok((footprint, peak)) = get_memory_usage(pid) {
            writeln!(memory_writer, "{},{},{}", elapsed_ms, footprint, peak)?;
            memory_writer.flush()?;
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
