//! Shared fixtures and the thread-count list for the `stages` and `aggregate` bench binaries.

use cellsmith::model::{parse_spec, Cell};

/// Parse the example spec's cells (unanalysed), fresh for each benchmark iteration.
pub fn raw_cells() -> Vec<Cell> {
    parse_spec(include_str!("../../examples/cells.toml"))
        .unwrap()
        .cells
}

/// The environment variable naming the thread counts to measure at.
pub const THREADS_VAR: &str = "CELLSMITH_BENCH_THREADS";

/// The thread counts every target is measured at, as a comma-separated list in [`THREADS_VAR`]:
/// `CELLSMITH_BENCH_THREADS=1,2,4,8` measures those four widths, and `CELLSMITH_BENCH_THREADS=1`
/// measures the single-threaded point alone. A width is a point on one axis, not a mode — one thread
/// is where the sweep starts, not a separate kind of run.
///
/// Unset is one measurement with nothing pinned: the work runs on the global pool at whatever width
/// it was configured with, which is how the tool itself runs.
///
/// The list arrives through the environment because `criterion_main!` owns `argv` and rejects
/// arguments it does not recognise, so a bench cannot take one of its own.
pub fn thread_counts() -> Vec<Option<usize>> {
    let Ok(list) = std::env::var(THREADS_VAR) else {
        return vec![None];
    };
    let counts: Vec<Option<usize>> = list
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let n: usize = s
                .parse()
                .unwrap_or_else(|_| panic!("{THREADS_VAR}: {s:?} is not a thread count"));
            assert!(n > 0, "{THREADS_VAR}: a thread count is at least 1");
            Some(n)
        })
        .collect();
    if counts.is_empty() {
        vec![None]
    } else {
        counts
    }
}

/// How a thread count reads in a benchmark id.
pub fn label(n: Option<usize>) -> String {
    match n {
        Some(n) => format!("n{n}"),
        None => "default".to_string(),
    }
}

/// The pool a measurement runs in: one of width `n`, or none where nothing is pinned. Built once per
/// registration, outside the timed closure.
pub fn pool(n: Option<usize>) -> Option<rayon::ThreadPool> {
    n.map(|n| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build()
            .unwrap()
    })
}

/// Run `f` in `pool`, or on the global pool where nothing is pinned.
pub fn in_pool<R: Send>(pool: &Option<rayon::ThreadPool>, f: impl FnOnce() -> R + Send) -> R {
    match pool {
        Some(p) => p.install(f),
        None => f(),
    }
}
