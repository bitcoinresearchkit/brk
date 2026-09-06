use crate::{Result, group::Group, record::Record, routes};
use jiff::civil::DateTime;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct Analysis {
    pub files: Vec<PathBuf>,
    pub groups: BTreeMap<(String, u16), Group>,
    pub lines: usize,
    pub ignored: usize,
    pub malformed: usize,
    pub unmatched: usize,
    pub count: usize,
    pub errors: usize,
    pub first: Option<DateTime>,
    pub last: Option<DateTime>,
}

impl Analysis {
    pub fn read(files: &[PathBuf]) -> Result<Self> {
        let catalog = routes::catalog();
        let mut groups: BTreeMap<(String, u16), Group> = BTreeMap::new();
        let (mut lines, mut ignored, mut malformed, mut unmatched, mut count, mut errors) =
            (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
        let (mut first, mut last) = (None, None);
        for path in files {
            for line in BufReader::new(File::open(path)?).lines() {
                let line = line?;
                lines += 1;
                let record = match Record::parse(&line) {
                    Ok(Some(record)) => record,
                    Ok(None) => {
                        ignored += 1;
                        continue;
                    }
                    Err(()) => {
                        malformed += 1;
                        continue;
                    }
                };
                first = Some(first.map_or(record.timestamp, |t: jiff::civil::DateTime| {
                    t.min(record.timestamp)
                }));
                last = Some(last.map_or(record.timestamp, |t: jiff::civil::DateTime| {
                    t.max(record.timestamp)
                }));
                count += 1;
                errors += usize::from(record.status >= 400);
                let endpoint = routes::endpoint(&record.uri, &catalog);
                let matched = catalog.contains(&endpoint);
                if !matched {
                    unmatched += 1;
                }
                let group = groups
                    .entry((endpoint.to_string(), record.status))
                    .or_default();
                group.matched = matched;
                group.push(record);
            }
        }
        for group in groups.values_mut() {
            group.durations.sort_unstable();
        }
        Ok(Self {
            files: files.to_vec(),
            groups,
            lines,
            ignored,
            malformed,
            unmatched,
            count,
            errors,
            first,
            last,
        })
    }
}

#[cfg(test)]
#[path = "../tests/unit/analysis.rs"]
mod tests;
