//! Shared plumbing for the benchmark binaries: reading the gzipped corpora,
//! and the small statistics helpers every report uses.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

/// The directory holding `corpus/` and `results/`, whatever directory the
/// binary is started from.
pub fn bench_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every `*.jsonl.gz` file in a corpus directory, sorted by name.
pub fn corpus_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.to_string_lossy().ends_with(".jsonl.gz"))
        .collect();
    files.sort();
    files
}

/// The lines of one gzipped JSON-lines file, parsed.
pub fn read_jsonl(path: &Path) -> Vec<serde_json::Value> {
    let f = File::open(path).unwrap_or_else(|e| panic!("cannot open {}: {e}", path.display()));
    BufReader::new(GzDecoder::new(f))
        .lines()
        .map(|l| l.unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(&l).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .collect()
}

/// A counter keyed by label, printed in descending order.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Tally(pub BTreeMap<String, u64>);

impl Tally {
    pub fn add(&mut self, key: impl Into<String>) {
        *self.0.entry(key.into()).or_insert(0) += 1;
    }
    pub fn get(&self, key: &str) -> u64 {
        self.0.get(key).copied().unwrap_or(0)
    }
    pub fn total(&self) -> u64 {
        self.0.values().sum()
    }
    pub fn merge(&mut self, other: &Tally) {
        for (k, v) in &other.0 {
            *self.0.entry(k.clone()).or_insert(0) += v;
        }
    }
    pub fn sorted(&self) -> Vec<(String, u64)> {
        let mut v: Vec<(String, u64)> = self.0.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v
    }
}

/// Percentile of a sorted slice, nearest-rank.
pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let rank = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

pub fn pct(n: u64, d: u64) -> String {
    if d == 0 {
        "-".into()
    } else {
        format!("{:.1}%", 100.0 * n as f64 / d as f64)
    }
}
