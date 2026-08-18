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

/// The baseline and default plus the intermediate widths 1, 2, 4, 8 below the max, for a target whose
/// scaling they can show.
pub fn full_sweep() -> Vec<usize> {
    let m = max_threads();
    let mut v: Vec<usize> = [1, 2, 4, 8].into_iter().filter(|&n| n < m).collect();
    v.push(m);
    v
}

/// The two informative points every target has: the `n=1` baseline and the default width.
pub fn flat_sweep() -> Vec<usize> {
    let m = max_threads();
    if m == 1 {
        vec![1]
    } else {
        vec![1, m]
    }
}

/// Which thread counts one benchmark target is measured at.
///
/// Two independent flags decide it, and neither is a classification of the other: `heavy` is a property
/// of the cell — its machine is wide enough for the intermediate widths to show scaling — and `parallel`
/// is a property of the stage being measured. All four combinations are configurations a target can be
/// in, including a light cell on a serial stage.
///
/// The rule over them: the `n=1` baseline and the default width are informative for every target, since
/// one is the reference the other is read against. The intermediate widths say something only when the
/// stage parallelises *and* the cell is wide enough to scale — on a serial stage they re-measure the
/// same single-threaded work, and on a cheap cell they measure noise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sweep {
    /// The cell is one of [`HEAVY`].
    pub heavy: bool,
    /// The stage parallelises internally.
    pub parallel: bool,
}

impl Sweep {
    /// The sweep for `cell` on a stage that does or does not parallelise internally.
    pub fn of(cell: &str, parallel: bool) -> Sweep {
        Sweep {
            heavy: is_heavy(cell),
            parallel,
        }
    }

    /// The thread counts this sweep is measured at.
    pub fn points(self) -> Vec<usize> {
        if self.heavy && self.parallel {
            full_sweep()
        } else {
            flat_sweep()
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
