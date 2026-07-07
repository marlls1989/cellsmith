#![allow(dead_code)]
//! Shared fixtures and thread-sweep helpers for the `stages` and `aggregate` bench binaries. Each
//! binary uses only a subset of these; the blanket `dead_code` allow avoids per-binary dead-code
//! churn.

use cellsmith::model::{parse_spec, AnalysedCell, Cell};
use rayon::prelude::*;

/// Cells whose machine width makes them worth sweeping across the full thread range; the rest are
/// cheap enough that only the `n=1` baseline and the default parallelism are informative.
pub const HEAVY: [&str; 2] = ["ICM", "RACELEM21"];

/// Whether `name` is one of the [`HEAVY`] cells.
pub fn is_heavy(name: &str) -> bool {
    HEAVY.contains(&name)
}

/// Parse the example spec's cells (unanalysed), fresh for each benchmark iteration.
pub fn raw_cells() -> Vec<Cell> {
    parse_spec(include_str!("../../examples/cells.toml"))
        .unwrap()
        .cells
}

/// Analyse every cell in parallel; a fixture-prep helper for stages that need an already-analysed
/// cell as input, run outside the timed region.
pub fn analyse_all(cells: &[Cell]) -> Vec<AnalysedCell> {
    cells.par_iter().map(|c| c.analyse().unwrap()).collect()
}

/// The rayon global thread pool's configured width.
pub fn max_threads() -> usize {
    rayon::current_num_threads()
}

/// The full thread sweep for heavy, parallel-capable benchmarks: 1, 2, 4, 8 (each below the max) plus
/// the max itself.
pub fn full_sweep() -> Vec<usize> {
    let m = max_threads();
    let mut v: Vec<usize> = [1, 2, 4, 8].into_iter().filter(|&n| n < m).collect();
    v.push(m);
    v
}

/// A flat two-point sweep (baseline `n=1` and the max) for heavy but non-parallel benchmarks.
pub fn flat_sweep() -> Vec<usize> {
    let m = max_threads();
    if m == 1 {
        vec![1]
    } else {
        vec![1, m]
    }
}

/// Pick the thread sweep for a stage given whether it is internally parallel and whether it is
/// [`HEAVY`]: light stages only run at the max thread count, heavy stages sweep 1..max (fully if
/// parallel, flat otherwise).
pub fn sweep(parallel: bool, heavy: bool) -> Vec<usize> {
    match (parallel, heavy) {
        (_, false) => vec![max_threads()],
        (true, true) => full_sweep(),
        (false, true) => flat_sweep(),
    }
}

/// Build a scoped rayon thread pool of width `n`, for measuring a stage under a specific thread
/// count.
pub fn pool(n: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(n)
        .build()
        .unwrap()
}
