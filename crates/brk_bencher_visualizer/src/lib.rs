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

// Dark theme colors
const BG_COLOR: RGBColor = RGBColor(18, 18, 24);
const TEXT_COLOR: RGBColor = RGBColor(230, 230, 240);
const CHART_COLORS: [RGBColor; 6] = [
    RGBColor(255, 99, 132),  // Pink/Red
    RGBColor(54, 162, 235),  // Blue
    RGBColor(75, 192, 192),  // Teal
    RGBColor(255, 206, 86),  // Yellow
    RGBColor(153, 102, 255), // Purple
    RGBColor(255, 159, 64),  // Orange
];

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
            .and_then(|p| p.parent())
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
        let output_path = crate_path.join("disk_usage_chart.svg");

        let root = SVGBackend::new(&output_path, (1200, 700)).into_drawing_area();
        root.fill(&BG_COLOR)?;

        let max_time = runs
            .iter()
            .flat_map(|r| r.data.iter().map(|d| d.timestamp_ms))
            .max()
            .unwrap_or(1000);

        let max_value = runs
            .iter()
            .flat_map(|r| r.data.iter().map(|d| d.value))
            .fold(0.0_f64, f64::max);

        // Convert to seconds and GB
        let max_time_s = (max_time as f64) / 1000.0;
        let max_value_gb = max_value / 1024.0;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — Disk Usage", crate_name),
                ("SF Mono", 24).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(55)
            .y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_s * 1.05, 0.0..max_value_gb * 1.1)?;

        chart
            .configure_mesh()
            .disable_mesh()
            .x_desc("Time (s)")
            .y_desc("Disk Usage (GB)")
            .x_label_offset(10)
            .y_label_offset(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_label_formatter(&|y| format!("{:.2}", y))
            .x_labels(8)
            .y_labels(6)
            .x_label_style(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.7)))
            .y_label_style(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.7)))
            .axis_style(TEXT_COLOR.mix(0.3))
            .draw()?;

        for (idx, run) in runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];

            chart
                .draw_series(LineSeries::new(
                    run.data
                        .iter()
                        .map(|d| (d.timestamp_ms as f64 / 1000.0, d.value / 1024.0)),
                    color.stroke_width(2),
                ))?
                .label(&run.run_id)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2))
                });
        }

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .label_font(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.9)))
            .background_style(BG_COLOR.mix(0.98))
            .border_style(BG_COLOR)
            .margin(10)
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
        let output_path = crate_path.join("memory_footprint_chart.svg");

        let root = SVGBackend::new(&output_path, (1200, 700)).into_drawing_area();
        root.fill(&BG_COLOR)?;

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

        // Convert to seconds and GB
        let max_time_s = (max_time as f64) / 1000.0;
        let max_value_gb = max_value / 1024.0;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — Memory Footprint", crate_name),
                ("SF Mono", 24).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(55)
            .y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_s * 1.05, 0.0..max_value_gb * 1.1)?;

        chart
            .configure_mesh()
            .disable_mesh()
            .x_desc("Time (s)")
            .y_desc("Memory (GB)")
            .x_label_offset(10)
            .y_label_offset(10)
            .x_label_formatter(&|x| format!("{:.1}", x))
            .y_label_formatter(&|y| format!("{:.2}", y))
            .x_labels(8)
            .y_labels(6)
            .x_label_style(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.7)))
            .y_label_style(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.7)))
            .axis_style(TEXT_COLOR.mix(0.3))
            .draw()?;

        for (idx, (run_id, footprint_data, peak_data)) in enhanced_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];

            // Draw footprint line (solid)
            chart
                .draw_series(LineSeries::new(
                    footprint_data
                        .iter()
                        .map(|d| (d.timestamp_ms as f64 / 1000.0, d.value / 1024.0)),
                    color.stroke_width(2),
                ))?
                .label(format!("{} (current)", run_id))
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2))
                });

            // Draw peak line (dashed, slightly transparent)
            let dashed_color = color.mix(0.5);
            chart
                .draw_series(
                    peak_data
                        .iter()
                        .map(|d| (d.timestamp_ms as f64 / 1000.0, d.value / 1024.0))
                        .zip(
                            peak_data
                                .iter()
                                .skip(1)
                                .map(|d| (d.timestamp_ms as f64 / 1000.0, d.value / 1024.0)),
                        )
                        .enumerate()
                        .filter(|(i, _)| i % 2 == 0) // Create dashed effect
                        .map(|(_, (p1, p2))| {
                            PathElement::new(vec![p1, p2], dashed_color.stroke_width(2))
                        }),
                )?
                .label(format!("{} (peak)", run_id))
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 10, y), (x + 20, y)],
                        dashed_color.stroke_width(2),
                    )
                });
        }

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .label_font(("SF Mono", 16).into_font().color(&TEXT_COLOR.mix(0.9)))
            .background_style(BG_COLOR.mix(0.98))
            .border_style(BG_COLOR)
            .margin(10)
            .draw()?;

        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }
}
