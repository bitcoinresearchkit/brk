use bitviewd_bench_visualizer::Visualizer;
use std::{fs, path::Path};

fn fixture(root: &Path) {
    let base = root.join("benches/bitviewd");
    for (name, scale) in [("run-a", 1), ("run-b", 2)] {
        let run = base.join(name);
        fs::create_dir_all(&run).unwrap();
        for (file, header, rows) in [
            (
                "disk.csv",
                "timestamp,value",
                format!("0,100\n1000,{}\n2000,300\n", 200 * scale),
            ),
            (
                "progress.csv",
                "timestamp,value",
                format!("0,0\n1000,{}\n2000,100\n", 20 * scale),
            ),
            (
                "memory.csv",
                "timestamp,current,peak",
                "0,20,30\n1000,50,80\n2000,40,100\n".into(),
            ),
            (
                "io.csv",
                "timestamp,read,write",
                "0,10,20\n1000,40,50\n2000,80,90\n".into(),
            ),
        ] {
            fs::write(run.join(file), format!("{header}\n{rows}")).unwrap();
        }
    }
}

#[test]
fn generates_all_combined_and_individual_charts() {
    let root = tempfile::tempdir().unwrap();
    fixture(root.path());
    Visualizer::new(root.path()).generate().unwrap();
    let base = root.path().join("benches/bitviewd");
    for dir in [base.clone(), base.join("run-a"), base.join("run-b")] {
        for name in ["disk", "memory", "progress", "io_read", "io_write"] {
            let svg = fs::read_to_string(dir.join(format!("{name}.svg"))).unwrap();
            assert!(svg.starts_with("<svg"));
            assert!(svg.contains("run-a") || svg.contains("run-b"));
            assert!(!svg.contains("NaN"));
        }
    }
}
