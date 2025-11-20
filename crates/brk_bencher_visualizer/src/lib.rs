use plotters::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
    slice,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const FONT: &str = "monospace";
const FONT_SIZE: i32 = 20;
const FONT_SIZE_BIG: i32 = 30;
const SIZE: (u32, u32) = (2000, 1000);

macro_rules! configure_chart_mesh {
    ($chart:expr, $x_desc:expr, $y_desc:expr, $y_formatter:expr) => {
        $chart
            .configure_mesh()
            .disable_mesh()
            .x_desc($x_desc)
            .y_desc($y_desc)
            .x_label_formatter(&|x| format!("{:.0}", x))
            .y_label_formatter(&$y_formatter)
            .x_labels(12)
            .y_labels(10)
            .x_label_style((FONT, FONT_SIZE).into_font().color(&TEXT_COLOR.mix(0.7)))
            .y_label_style((FONT, FONT_SIZE).into_font().color(&TEXT_COLOR.mix(0.7)))
            .axis_style(TEXT_COLOR.mix(0.3))
            .draw()?
    };
}

#[derive(Debug, Clone)]
struct DataPoint {
    timestamp_ms: u64,
    value: f64,
}

#[derive(Debug, Clone)]
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

// Time window buffer in milliseconds
const TIME_BUFFER_MS: u64 = 10_000;

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

    pub fn generate_all_charts(&self) -> Result<()> {
        let benches_dir = self.workspace_root.join("benches");

        if !benches_dir.exists() {
            return Err("Benches directory does not exist".into());
        }

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

    fn generate_crate_charts(&self, crate_path: &Path, crate_name: &str) -> Result<()> {
        let disk_runs = self.read_benchmark_runs(crate_path, "disk.csv")?;
        let memory_runs = self.read_benchmark_runs(crate_path, "memory.csv")?;
        let progress_runs = self.read_benchmark_runs(crate_path, "progress.csv")?;
        let io_runs = self.read_benchmark_runs(crate_path, "io.csv")?;

        // Generate combined charts (all runs together)
        if !disk_runs.is_empty() {
            self.generate_disk_chart(crate_path, crate_name, &disk_runs)?;
        }

        if !memory_runs.is_empty() {
            self.generate_memory_chart(crate_path, crate_name, &memory_runs)?;
        }

        if !progress_runs.is_empty() {
            self.generate_progress_chart(crate_path, crate_name, &progress_runs)?;
        }

        if !io_runs.is_empty() {
            self.generate_io_read_chart(crate_path, crate_name, &io_runs)?;
            self.generate_io_write_chart(crate_path, crate_name, &io_runs)?;
        }

        // Generate individual charts for each run
        for run in &disk_runs {
            let run_path = crate_path.join(&run.run_id);
            self.generate_disk_chart(&run_path, crate_name, slice::from_ref(run))?;
        }

        for run in &memory_runs {
            let run_path = crate_path.join(&run.run_id);
            self.generate_memory_chart(&run_path, crate_name, slice::from_ref(run))?;
        }

        for run in &progress_runs {
            let run_path = crate_path.join(&run.run_id);
            self.generate_progress_chart(&run_path, crate_name, slice::from_ref(run))?;
        }

        for run in &io_runs {
            let run_path = crate_path.join(&run.run_id);
            self.generate_io_read_chart(&run_path, crate_name, slice::from_ref(run))?;
            self.generate_io_write_chart(&run_path, crate_name, slice::from_ref(run))?;
        }

        Ok(())
    }

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

                // Skip directories that start with underscore or contain only numbers
                if run_id.starts_with('_') || run_id.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }

                let csv_path = run_path.join(filename);

                if csv_path.exists()
                    && let Ok(data) = Self::read_csv(&csv_path, 2)
                {
                    runs.push(BenchmarkRun { run_id, data });
                }
            }
        }

        Ok(runs)
    }

    fn read_csv(path: &Path, expected_columns: usize) -> Result<Vec<DataPoint>> {
        let content = fs::read_to_string(path)?;
        let mut data = Vec::new();

        for (i, line) in content.lines().enumerate() {
            if i == 0 {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= expected_columns
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

    // Helper methods

    fn format_bytes(bytes: f64) -> (f64, &'static str) {
        const KIB: f64 = 1024.0;
        const MIB: f64 = 1024.0 * 1024.0;
        const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

        if bytes >= GIB {
            (bytes / GIB, "GiB")
        } else if bytes >= MIB {
            (bytes / MIB, "MiB")
        } else if bytes >= KIB {
            (bytes / KIB, "KiB")
        } else {
            (bytes, "bytes")
        }
    }

    fn format_time(time_s: f64) -> (f64, &'static str, &'static str) {
        const MINUTE: f64 = 60.0;
        const HOUR: f64 = 3600.0;

        // Only use larger units if the value would be >= 2 (to avoid decimals)
        if time_s >= HOUR * 2.0 {
            (time_s / HOUR, "h", "Time (h)")
        } else if time_s >= MINUTE * 2.0 {
            (time_s / MINUTE, "min", "Time (min)")
        } else {
            (time_s, "s", "Time (s)")
        }
    }

    fn format_axis_number(value: f64) -> String {
        if value >= 1000.0 {
            let k_value = value / 1000.0;
            // Show decimals only if needed
            if k_value.fract() == 0.0 || k_value >= 100.0 {
                format!("{:.0}k", k_value)
            } else if k_value >= 10.0 {
                format!("{:.1}k", k_value)
            } else {
                format!("{:.2}k", k_value)
            }
        } else {
            format!("{:.0}", value)
        }
    }

    fn calculate_min_max_time(runs: &[BenchmarkRun]) -> u64 {
        runs.iter()
            .filter_map(|r| r.data.iter().map(|d| d.timestamp_ms).max())
            .min()
            .unwrap_or(1000)
    }

    fn calculate_max_value(runs: &[BenchmarkRun]) -> f64 {
        runs.iter()
            .flat_map(|r| r.data.iter().map(|d| d.value))
            .fold(0.0_f64, f64::max)
    }

    fn trim_runs_to_time_window(runs: &[BenchmarkRun], max_time_ms: u64) -> Vec<BenchmarkRun> {
        runs.iter()
            .map(|run| BenchmarkRun {
                run_id: run.run_id.clone(),
                data: run
                    .data
                    .iter()
                    .filter(|d| d.timestamp_ms <= max_time_ms)
                    .cloned()
                    .collect(),
            })
            .collect()
    }

    fn draw_line_series<'a, DB: DrawingBackend, I>(
        chart: &mut ChartContext<
            'a,
            DB,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
        data: I,
        label: &str,
        color: RGBColor,
    ) -> Result<()>
    where
        I: Iterator<Item = (f64, f64)> + Clone,
        DB::ErrorType: 'static,
    {
        chart
            .draw_series(LineSeries::new(data, color.stroke_width(1)))?
            .label(label)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(1))
            });
        Ok(())
    }

    fn configure_series_labels<'a, DB: DrawingBackend + 'a>(
        chart: &mut ChartContext<
            'a,
            DB,
            Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
    ) -> Result<()>
    where
        DB::ErrorType: 'static,
    {
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .label_font((FONT, FONT_SIZE).into_font().color(&TEXT_COLOR.mix(0.9)))
            .background_style(BG_COLOR.mix(0.98))
            .border_style(BG_COLOR)
            .margin(10)
            .draw()?;
        Ok(())
    }

    // Chart generation methods

    fn generate_disk_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("disk.svg");
        let root = SVGBackend::new(&output_path, SIZE).into_drawing_area();
        root.fill(&BG_COLOR)?;

        // Calculate time window based on shortest run + buffer
        let min_max_time_ms = Self::calculate_min_max_time(runs) + TIME_BUFFER_MS;
        let max_time_s = (min_max_time_ms as f64) / 1000.0;

        // Trim all runs to the same time window
        let trimmed_runs = Self::trim_runs_to_time_window(runs, min_max_time_ms);

        let max_value = Self::calculate_max_value(&trimmed_runs);
        let (max_value_scaled, unit) = Self::format_bytes(max_value);
        let scale_factor = max_value / max_value_scaled;

        // Format time based on duration
        let (max_time_scaled, _time_unit, time_label) = Self::format_time(max_time_s);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — Disk Usage", crate_name),
                (FONT, FONT_SIZE_BIG).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(50)
            .margin_left(50)
            .right_y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_scaled * 1.025, 0.0..max_value_scaled * 1.1)?;

        configure_chart_mesh!(
            chart,
            time_label,
            format!("Disk Usage ({})", unit),
            |y: &f64| format!("{:.2}", y)
        );

        for (idx, run) in trimmed_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];
            let time_divisor = max_time_s / max_time_scaled;
            Self::draw_line_series(
                &mut chart,
                run.data.iter().map(|d| {
                    (
                        d.timestamp_ms as f64 / 1000.0 / time_divisor,
                        d.value / scale_factor,
                    )
                }),
                &run.run_id,
                color,
            )?;
        }

        Self::configure_series_labels(&mut chart)?;
        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }

    fn generate_memory_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("memory.svg");
        let root = SVGBackend::new(&output_path, SIZE).into_drawing_area();
        root.fill(&BG_COLOR)?;

        // Calculate time window based on shortest run + buffer
        let min_max_time_ms = Self::calculate_min_max_time(runs) + TIME_BUFFER_MS;
        let max_time_s = (min_max_time_ms as f64) / 1000.0;

        // Read memory CSV files which have 3 columns: timestamp, footprint, peak
        let enhanced_runs = self.read_memory_data(crate_path, runs)?;

        // Trim enhanced runs to the same time window
        let trimmed_enhanced_runs: Vec<_> = enhanced_runs
            .into_iter()
            .map(|(run_id, footprint, peak)| {
                let trimmed_footprint: Vec<_> = footprint
                    .into_iter()
                    .filter(|d| d.timestamp_ms <= min_max_time_ms)
                    .collect();
                let trimmed_peak: Vec<_> = peak
                    .into_iter()
                    .filter(|d| d.timestamp_ms <= min_max_time_ms)
                    .collect();
                (run_id, trimmed_footprint, trimmed_peak)
            })
            .collect();

        let max_value = trimmed_enhanced_runs
            .iter()
            .flat_map(|(_, f, p)| f.iter().chain(p.iter()).map(|d| d.value))
            .fold(0.0_f64, f64::max);

        let (max_value_scaled, unit) = Self::format_bytes(max_value);
        let scale_factor = max_value / max_value_scaled;

        // Format time based on duration
        let (max_time_scaled, _time_unit, time_label) = Self::format_time(max_time_s);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — Memory", crate_name),
                (FONT, FONT_SIZE_BIG).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(50)
            .margin_left(50)
            .right_y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_scaled * 1.025, 0.0..max_value_scaled * 1.1)?;

        configure_chart_mesh!(
            chart,
            time_label,
            format!("Memory ({})", unit),
            |y: &f64| format!("{:.2}", y)
        );

        let time_divisor = max_time_s / max_time_scaled;

        for (idx, (run_id, footprint_data, peak_data)) in trimmed_enhanced_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];

            Self::draw_line_series(
                &mut chart,
                footprint_data.iter().map(|d| {
                    (
                        d.timestamp_ms as f64 / 1000.0 / time_divisor,
                        d.value / scale_factor,
                    )
                }),
                &format!("{} (current)", run_id),
                color,
            )?;

            // Draw peak line (dashed) - inline to handle time_divisor
            let dashed_color = color.mix(0.5);
            chart
                .draw_series(
                    peak_data
                        .iter()
                        .map(|d| {
                            (
                                d.timestamp_ms as f64 / 1000.0 / time_divisor,
                                d.value / scale_factor,
                            )
                        })
                        .zip(peak_data.iter().skip(1).map(|d| {
                            (
                                d.timestamp_ms as f64 / 1000.0 / time_divisor,
                                d.value / scale_factor,
                            )
                        }))
                        .enumerate()
                        .filter(|(i, _)| i % 2 == 0)
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

        Self::configure_series_labels(&mut chart)?;
        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn read_memory_data(
        &self,
        crate_path: &Path,
        runs: &[BenchmarkRun],
    ) -> Result<Vec<(String, Vec<DataPoint>, Vec<DataPoint>)>> {
        let mut enhanced_runs = Vec::new();

        for run in runs {
            // For individual charts, crate_path is already the run folder
            // For combined charts, we need to append run_id
            let direct_path = crate_path.join("memory.csv");
            let nested_path = crate_path.join(&run.run_id).join("memory.csv");
            let csv_path = if direct_path.exists() {
                direct_path
            } else {
                nested_path
            };
            if let Ok(content) = fs::read_to_string(&csv_path) {
                let mut footprint_data = Vec::new();
                let mut peak_data = Vec::new();

                for (i, line) in content.lines().enumerate() {
                    if i == 0 {
                        continue;
                    }

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

        Ok(enhanced_runs)
    }

    #[allow(clippy::type_complexity)]
    fn read_io_data(
        &self,
        crate_path: &Path,
        runs: &[BenchmarkRun],
    ) -> Result<Vec<(String, Vec<DataPoint>, Vec<DataPoint>)>> {
        let mut io_runs = Vec::new();

        for run in runs {
            // For individual charts, crate_path is already the run folder
            // For combined charts, we need to append run_id
            let direct_path = crate_path.join("io.csv");
            let nested_path = crate_path.join(&run.run_id).join("io.csv");
            let csv_path = if direct_path.exists() {
                direct_path
            } else {
                nested_path
            };
            if let Ok(content) = fs::read_to_string(&csv_path) {
                let mut read_data = Vec::new();
                let mut write_data = Vec::new();

                for (i, line) in content.lines().enumerate() {
                    if i == 0 {
                        continue;
                    }

                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 3
                        && let (Ok(timestamp_ms), Ok(bytes_read), Ok(bytes_written)) = (
                            parts[0].parse::<u64>(),
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                        )
                    {
                        read_data.push(DataPoint {
                            timestamp_ms,
                            value: bytes_read,
                        });
                        write_data.push(DataPoint {
                            timestamp_ms,
                            value: bytes_written,
                        });
                    }
                }

                io_runs.push((run.run_id.clone(), read_data, write_data));
            }
        }

        Ok(io_runs)
    }

    fn generate_io_read_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("io_read.svg");
        let root = SVGBackend::new(&output_path, SIZE).into_drawing_area();
        root.fill(&BG_COLOR)?;

        // Calculate time window based on shortest run + buffer
        let min_max_time_ms = Self::calculate_min_max_time(runs) + TIME_BUFFER_MS;
        let max_time_s = (min_max_time_ms as f64) / 1000.0;

        // Read I/O CSV files which have 3 columns: timestamp, bytes_read, bytes_written
        let io_runs = self.read_io_data(crate_path, runs)?;

        // Trim I/O runs to the same time window and extract only read data
        let trimmed_io_runs: Vec<_> = io_runs
            .into_iter()
            .map(|(run_id, read_data, _write_data)| {
                let trimmed_read: Vec<_> = read_data
                    .into_iter()
                    .filter(|d| d.timestamp_ms <= min_max_time_ms)
                    .collect();
                (run_id, trimmed_read)
            })
            .collect();

        let max_value = trimmed_io_runs
            .iter()
            .flat_map(|(_, data)| data.iter().map(|d| d.value))
            .fold(0.0_f64, f64::max);

        let (max_value_scaled, unit) = Self::format_bytes(max_value);
        let scale_factor = max_value / max_value_scaled;

        // Format time based on duration
        let (max_time_scaled, _time_unit, time_label) = Self::format_time(max_time_s);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — I/O Read", crate_name),
                (FONT, FONT_SIZE_BIG).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(50)
            .margin_left(50)
            .right_y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_scaled * 1.025, 0.0..max_value_scaled * 1.1)?;

        configure_chart_mesh!(
            chart,
            time_label,
            format!("Bytes Read ({})", unit),
            |y: &f64| format!("{:.2}", y)
        );

        let time_divisor = max_time_s / max_time_scaled;

        for (idx, (run_id, read_data)) in trimmed_io_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];

            Self::draw_line_series(
                &mut chart,
                read_data.iter().map(|d| {
                    (
                        d.timestamp_ms as f64 / 1000.0 / time_divisor,
                        d.value / scale_factor,
                    )
                }),
                run_id,
                color,
            )?;
        }

        Self::configure_series_labels(&mut chart)?;
        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }

    fn generate_io_write_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("io_write.svg");
        let root = SVGBackend::new(&output_path, SIZE).into_drawing_area();
        root.fill(&BG_COLOR)?;

        // Calculate time window based on shortest run + buffer
        let min_max_time_ms = Self::calculate_min_max_time(runs) + TIME_BUFFER_MS;
        let max_time_s = (min_max_time_ms as f64) / 1000.0;

        // Read I/O CSV files which have 3 columns: timestamp, bytes_read, bytes_written
        let io_runs = self.read_io_data(crate_path, runs)?;

        // Trim I/O runs to the same time window and extract only write data
        let trimmed_io_runs: Vec<_> = io_runs
            .into_iter()
            .map(|(run_id, _read_data, write_data)| {
                let trimmed_write: Vec<_> = write_data
                    .into_iter()
                    .filter(|d| d.timestamp_ms <= min_max_time_ms)
                    .collect();
                (run_id, trimmed_write)
            })
            .collect();

        let max_value = trimmed_io_runs
            .iter()
            .flat_map(|(_, data)| data.iter().map(|d| d.value))
            .fold(0.0_f64, f64::max);

        let (max_value_scaled, unit) = Self::format_bytes(max_value);
        let scale_factor = max_value / max_value_scaled;

        // Format time based on duration
        let (max_time_scaled, _time_unit, time_label) = Self::format_time(max_time_s);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — I/O Write", crate_name),
                (FONT, FONT_SIZE_BIG).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(50)
            .margin_left(50)
            .right_y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_scaled * 1.025, 0.0..max_value_scaled * 1.1)?;

        configure_chart_mesh!(
            chart,
            time_label,
            format!("Bytes Written ({})", unit),
            |y: &f64| format!("{:.2}", y)
        );

        let time_divisor = max_time_s / max_time_scaled;

        for (idx, (run_id, write_data)) in trimmed_io_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];

            Self::draw_line_series(
                &mut chart,
                write_data.iter().map(|d| {
                    (
                        d.timestamp_ms as f64 / 1000.0 / time_divisor,
                        d.value / scale_factor,
                    )
                }),
                run_id,
                color,
            )?;
        }

        Self::configure_series_labels(&mut chart)?;
        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }

    fn generate_progress_chart(
        &self,
        crate_path: &Path,
        crate_name: &str,
        runs: &[BenchmarkRun],
    ) -> Result<()> {
        let output_path = crate_path.join("progress.svg");
        let root = SVGBackend::new(&output_path, SIZE).into_drawing_area();
        root.fill(&BG_COLOR)?;

        // Calculate time window based on shortest run + buffer
        let min_max_time_ms = Self::calculate_min_max_time(runs) + TIME_BUFFER_MS;
        let max_time_s = (min_max_time_ms as f64) / 1000.0;

        // Trim all runs to the same time window
        let trimmed_runs = Self::trim_runs_to_time_window(runs, min_max_time_ms);

        let max_block = Self::calculate_max_value(&trimmed_runs);

        // Format time based on duration
        let (max_time_scaled, _time_unit, time_label) = Self::format_time(max_time_s);

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{} — Progress", crate_name),
                (FONT, FONT_SIZE_BIG).into_font().color(&TEXT_COLOR),
            )
            .margin(20)
            .x_label_area_size(50)
            .margin_left(50)
            .right_y_label_area_size(75)
            .build_cartesian_2d(0.0..max_time_scaled * 1.025, 0.0..max_block * 1.1)?;

        chart
            .configure_mesh()
            .disable_mesh()
            .x_desc(time_label)
            .y_desc("Block Number")
            .x_label_formatter(&|x| Self::format_axis_number(*x))
            .y_label_formatter(&|y| Self::format_axis_number(*y))
            .x_labels(12)
            .y_labels(10)
            .x_label_style((FONT, FONT_SIZE).into_font().color(&TEXT_COLOR.mix(0.7)))
            .y_label_style((FONT, FONT_SIZE).into_font().color(&TEXT_COLOR.mix(0.7)))
            .axis_style(TEXT_COLOR.mix(0.3))
            .draw()?;

        let time_divisor = max_time_s / max_time_scaled;

        for (idx, run) in trimmed_runs.iter().enumerate() {
            let color = CHART_COLORS[idx % CHART_COLORS.len()];
            Self::draw_line_series(
                &mut chart,
                run.data
                    .iter()
                    .map(|d| (d.timestamp_ms as f64 / 1000.0 / time_divisor, d.value)),
                &run.run_id,
                color,
            )?;
        }

        Self::configure_series_labels(&mut chart)?;
        root.present()?;
        println!("Generated: {}", output_path.display());
        Ok(())
    }
}
