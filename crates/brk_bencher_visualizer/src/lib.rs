use plotters::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct DataPoint {
    timestamp_ms: u64,
    value: f64,
}

#[derive(Debug)]
struct BenchmarkRun {
    run_id: String,
    data: Vec<DataPoint>,
}

pub struct Visualizer {
    workspace_root: PathBuf,
}

impl Visualizer {
    pub fn new(workspace_root: impl AsRef<Path>) -> Self {
        Self {
            workspace_root: workspace_root.as_ref().to_path_buf(),
        }
    }

    pub fn from_cargo_env() -> Result<Self> {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("Failed to find workspace root")?
            .to_path_buf();
        Ok(Self { workspace_root })
    }

    /// Generate all charts for all crates in the benches directory
    pub fn generate_all_charts(&self) -> Result<()> {
        let benches_dir = self.workspace_root.join("benches");

        if !benches_dir.exists() {
            return Err("Benches directory does not exist".into());
        }

        // Iterate through each crate directory
        for entry in fs::read_dir(&benches_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let crate_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or("Invalid crate name")?;

                println!("Generating charts for crate: {}", crate_name);
                self.generate_crate_charts(&path, crate_name)?;
            }
        }

        Ok(())
    }

    /// Generate charts for a specific crate
    fn generate_crate_charts(&self, crate_path: &Path, crate_name: &str) -> Result<()> {
        // Read all benchmark runs for this crate
        let disk_runs = self.read_benchmark_runs(crate_path, "disk_usage.csv")?;
        let memory_runs = self.read_benchmark_runs(crate_path, "memory_footprint.csv")?;

        if !disk_runs.is_empty() {
            self.generate_disk_chart(crate_path, crate_name, &disk_runs)?;
        }

        if !memory_runs.is_empty() {
            self.generate_memory_chart(crate_path, crate_name, &memory_runs)?;
        }

        Ok(())
    }

    /// Read all benchmark runs from subdirectories
    fn read_benchmark_runs(&self, crate_path: &Path, filename: &str) -> Result<Vec<BenchmarkRun>> {
        let mut runs = Vec::new();

        for entry in fs::read_dir(crate_path)? {
            let entry = entry?;
            let run_path = entry.path();

            if run_path.is_dir() {
                let run_id = run_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or("Invalid run ID")?
                    .to_string();

                let csv_path = run_path.join(filename);

                if csv_path.exists()
                    && let Ok(data) = self.read_csv(&csv_path)
                {
                    runs.push(BenchmarkRun { run_id, data });
                }
            }
        }

        Ok(runs)
    }

    /// Read a CSV file and parse data points
    fn read_csv(&self, path: &Path) -> Result<Vec<DataPoint>> {
        let content = fs::read_to_string(path)?;
        let mut data = Vec::new();

        for (i, line) in content.lines().enumerate() {
            if i == 0 {
                continue;
            } // Skip header

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2
                && let (Ok(timestamp_ms), Ok(value)) =
                    (parts[0].parse::<u64>(), parts[1].parse::<f64>())
            {
                data.push(DataPoint {
                    timestamp_ms,
                    value,
                });
            }
        }

        Ok(data)
    }

    /// Generate disk usage chart
    fn generate_disk_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("disk_usage_chart.png");

        let root = BitMapBackend::new(&output_path, (1200, 800)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_time = runs
            .iter()
            .flat_map(|r| r.data.iter().map(|d| d.timestamp_ms))
            .max()
            .unwrap_or(1000);

        let max_value = runs
            .iter()
            .flat_map(|r| r.data.iter().map(|d| d.value))
            .fold(0.0_f64, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} - Disk Usage", crate_name),
                ("sans-serif", 40).into_font(),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0u64..max_time, 0.0..max_value * 1.1)?;

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .y_desc("Disk Usage (MB)")
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &YELLOW];

        for (idx, run) in runs.iter().enumerate() {
            let color = colors[idx % colors.len()];

            chart
                .draw_series(LineSeries::new(
                    run.data.iter().map(|d| (d.timestamp_ms, d.value)),
                    color,
                ))?
                .label(&run.run_id)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }

    /// Generate memory footprint chart
    fn generate_memory_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("memory_footprint_chart.png");

        let root = BitMapBackend::new(&output_path, (1200, 800)).into_drawing_area();
        root.fill(&WHITE)?;

        // Read memory CSV files which have 3 columns: timestamp, footprint, peak
        let mut enhanced_runs = Vec::new();

        for run in runs {
            // Re-read the CSV to get both footprint and peak values
            let csv_path = crate_path.join(&run.run_id).join("memory_footprint.csv");
            if let Ok(content) = fs::read_to_string(&csv_path) {
                let mut footprint_data = Vec::new();
                let mut peak_data = Vec::new();

                for (i, line) in content.lines().enumerate() {
                    if i == 0 {
                        continue;
                    } // Skip header

                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 3
                        && let (Ok(timestamp_ms), Ok(footprint), Ok(peak)) = (
                            parts[0].parse::<u64>(),
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                        )
                    {
                        footprint_data.push(DataPoint {
                            timestamp_ms,
                            value: footprint,
                        });
                        peak_data.push(DataPoint {
                            timestamp_ms,
                            value: peak,
                        });
                    }
                }

                enhanced_runs.push((run.run_id.clone(), footprint_data, peak_data));
            }
        }

        let max_time = enhanced_runs
            .iter()
            .flat_map(|(_, f, p)| f.iter().chain(p.iter()).map(|d| d.timestamp_ms))
            .max()
            .unwrap_or(1000);

        let max_value = enhanced_runs
            .iter()
            .flat_map(|(_, f, p)| f.iter().chain(p.iter()).map(|d| d.value))
            .fold(0.0_f64, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} - Memory Footprint", crate_name),
                ("sans-serif", 40).into_font(),
            )
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0u64..max_time, 0.0..max_value * 1.1)?;

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .y_desc("Memory (MB)")
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &YELLOW];

        for (idx, (run_id, footprint_data, peak_data)) in enhanced_runs.iter().enumerate() {
            let color = colors[idx % colors.len()];

            // Draw footprint line (solid)
            chart
                .draw_series(LineSeries::new(
                    footprint_data.iter().map(|d| (d.timestamp_ms, d.value)),
                    color,
                ))?
                .label(format!("{} (current)", run_id))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));

            // Draw peak line (dashed)
            chart
                .draw_series(LineSeries::new(
                    peak_data.iter().map(|d| (d.timestamp_ms, d.value)),
                    color.stroke_width(2).filled(),
                ))?
                .label(format!("{} (peak)", run_id))
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 10, y), (x + 20, y)],
                        color.stroke_width(2),
                    )
                });
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }
}
