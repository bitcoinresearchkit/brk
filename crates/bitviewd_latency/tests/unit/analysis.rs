use super::*;

#[test]
fn aggregates_without_rendering_and_marks_unknown_paths() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("input.txt");
    std::fs::write(&path, "2026-09-04 12:00:02 - info  200 /api/tx/a 3ms\n2026-09-04 12:00:01 - info  200 /api/tx/b 1ms\n2026-09-04 12:00:03 - error 404 /unknown?x=1 2ms\n2026-09-04 12:00:04 - error 400 /unknown?x=2 1ms\n2026-09-04 12:00:00 - info  Starting\n2026-09-04 12:00:04 - info  200 /api broken\n").unwrap();
    let result = Analysis::read(&[path]).unwrap();
    assert_eq!((result.count, result.errors, result.unmatched), (4, 2, 2));
    assert_eq!((result.lines, result.ignored, result.malformed), (6, 1, 1));
    assert_eq!(result.first.unwrap().second(), 1);
    assert_eq!(result.last.unwrap().second(), 4);
    let group = &result.groups[&("/api/tx/{txid}".into(), 200)];
    assert!(group.matched);
    assert_eq!(group.durations, [1_000_000, 3_000_000]);
    assert_eq!(group.total, 4_000_000);
    assert!(!result.groups[&("/unknown".into(), 404)].matched);
    let report = crate::report::render(&result).unwrap();
    assert!(report.contains("| /unknown | 2 |"));
    assert!(report.contains("| /unknown [unmatched] | 1† |"));
}
