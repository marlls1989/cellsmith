//! Shared fixtures and thread-sweep helpers for the `stages` and `aggregate` bench binaries.

use cellsmith::model::{parse_spec, Cell};

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

/// The thread-count profile one benchmark target is measured over. There are three: a cell outside
/// [`HEAVY`] is cheap enough that only the default parallelism says anything, whatever the stage does,
/// and a [`HEAVY`] one is swept across the range — the full sweep when the stage parallelises
/// internally, the two-point baseline when it runs serially and the intermediate points would only
/// re-measure the same single-threaded work.
///
/// `dead_code` wants every variant constructed somewhere. This module is compiled separately into each
/// bench binary and neither reaches all of it: `aggregate` measures whole-pipeline targets, every one of
/// them internally parallel, so it never builds [`Profile::HeavySerial`]. Deleting the variant is not
/// available — `stages` needs it — and the form that would satisfy the lint is a shared bench-support
/// crate the two binaries both depend on, which is more machinery than two bench files warrant. The
/// attribute buys silence on that one per-binary asymmetry and nothing else; it is scoped to this enum,
/// so an unused helper elsewhere in the module still gets reported.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// A cell outside [`HEAVY`]: the default parallelism alone.
    Light,
    /// A [`HEAVY`] cell on an internally parallel stage.
    HeavyParallel,
    /// A [`HEAVY`] cell on a serial stage.
    HeavySerial,
}

impl Profile {
    /// The profile of `cell` on an internally parallel stage. A light cell collapses to
    /// [`Profile::Light`] whichever stage is being measured.
    pub fn parallel(cell: &str) -> Profile {
        if is_heavy(cell) {
            Profile::HeavyParallel
        } else {
            Profile::Light
        }
    }

    /// The profile of `cell` on a stage that does its work serially, collapsing the same way.
    ///
    /// Unreached by the `aggregate` bench binary, which has no serial target — see [`Profile`].
    #[allow(dead_code)]
    pub fn serial(cell: &str) -> Profile {
        if is_heavy(cell) {
            Profile::HeavySerial
        } else {
            Profile::Light
        }
    }

    /// The thread counts this profile is measured at.
    pub fn points(self) -> Vec<usize> {
        match self {
            Profile::Light => vec![max_threads()],
            Profile::HeavyParallel => full_sweep(),
            Profile::HeavySerial => flat_sweep(),
        }
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
