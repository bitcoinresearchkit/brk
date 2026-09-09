#![cfg(all(feature = "pco", feature = "serde"))]

use std::{hint::black_box, time::Instant};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, AnyVecWithWriter, BytesVec, Cursor, Database, Error, Formattable, ImportableVec,
    PcoVec, ReadableVec, Result, ValueWriter, Version, WritableVec,
};

struct CursorWriter<'a, V: ReadableVec<usize, u32>> {
    cursor: Cursor<'a, usize, u32, V>,
    end: usize,
}

impl<V: ReadableVec<usize, u32>> ValueWriter for CursorWriter<'_, V> {
    fn write_next(&mut self, output: &mut String) -> Result<()> {
        if self.cursor.position() >= self.end {
            return Err(Error::IteratorEnded);
        }
        self.cursor
            .next()
            .ok_or(Error::IteratorEnded)?
            .fmt_csv(output)?;
        Ok(())
    }
}

fn render<V: ReadableVec<usize, u32> + AnyVecWithWriter>(
    source: &V,
    start: usize,
    rows: usize,
    columns: usize,
    variant: usize,
) -> String {
    let mut output = String::with_capacity(rows * columns * 15);
    let end = start + rows;
    let budget_rows = if variant >= 3 {
        let rows = (1_048_576 / (columns * source.value_type_to_size_of())).max(1);
        if variant == 4 {
            let page = source.cursor_chunk_size().max(1);
            (rows / page).max(1) * page
        } else {
            rows
        }
    } else {
        0
    };
    let mut start = start;
    while start < end {
        let stop = if variant == 2 {
            ((start / 8192 + 1) * 8192).min(end)
        } else if variant >= 3 && rows > budget_rows {
            ((start / budget_rows + 1) * budget_rows).min(end)
        } else {
            end
        };
        let mut writers: Vec<Box<dyn ValueWriter + '_>> = (0..columns)
            .map(|_| {
                if variant == 1 {
                    let mut cursor = source.cursor();
                    cursor.advance(start);
                    Box::new(CursorWriter { cursor, end: stop }) as Box<dyn ValueWriter>
                } else {
                    source.create_writer(Some(start as i64), Some(stop as i64))
                }
            })
            .collect();
        for _ in start..stop {
            for (column, writer) in writers.iter_mut().enumerate() {
                if column != 0 {
                    output.push(',');
                }
                writer.write_next(&mut output).unwrap();
            }
            output.push('\n');
        }
        start = stop;
    }
    output
}

fn compare<V: ReadableVec<usize, u32> + AnyVecWithWriter>(name: &str, source: &V) {
    for columns in [2, 3, 8, 32] {
        for rows in [1, 4096, 90_000] {
            for start in [0, 17] {
                let expected = render(source, start, rows, columns, 0);
                for variant in [1, 2, 3, 4] {
                    assert_eq!(expected, render(source, start, rows, columns, variant));
                }
                let batches = (200_000 / (rows * columns)).clamp(1, 200);
                let mut times = [Vec::new(), Vec::new(), Vec::new()];
                for round in 0..12 {
                    for offset in 0..3 {
                        let sample = (round + offset) % 3;
                        let variant = [0, 3, 4][sample];
                        let began = Instant::now();
                        for _ in 0..batches {
                            black_box(render(black_box(source), start, rows, columns, variant));
                        }
                        if round >= 2 {
                            times[sample].push(began.elapsed() / batches as u32);
                        }
                    }
                }
                for samples in &mut times {
                    samples.sort_unstable();
                }
                eprintln!(
                    "{name} columns={columns} rows={rows} start={start}: collected {:?}, byte-budget {:?}, aligned {:?}",
                    times[0][5], times[1][5], times[2][5]
                );
            }
        }
    }
}

#[test]
#[ignore = "CSV collection/cursor/chunk comparison; warm local storage, no HTTP"]
fn benchmark_csv_writers() -> Result<()> {
    let temp = tempdir()?;
    let db = Database::open(temp.path())?;
    let mut raw = BytesVec::<usize, u32>::import(&db, "raw", Version::ONE)?;
    let mut compressed = PcoVec::<usize, u32>::import(&db, "compressed", Version::ONE)?;
    for index in 0..100_000_u32 {
        let value = 1_200_000_000 + index * 600;
        raw.push(value);
        compressed.push(value);
    }
    raw.write()?;
    compressed.write()?;
    compare("raw", &raw);
    compare("compressed", &compressed);
    Ok(())
}
