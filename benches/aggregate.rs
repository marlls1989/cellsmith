//! Whole-pipeline benchmarks: a single cell's full `analyse()` and the whole-crate run across all
//! example cells, each measured at every one of [`common::thread_counts`] with the pool built once
//! per registration.

mod common;

use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, CellArcs, Deck};
use cellsmith::emit::liberty::{cell_liberty, library_liberty};
use cellsmith::emit::verilog::{cell_verilog, Verilog};
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

/// One target per example cell: the full `analyse()` pipeline, which is internally parallel.
fn bench_cell_analyse(c: &mut Criterion) {
    let mut g = c.benchmark_group("whole_cell");
    let cells = common::raw_cells();
    for cell in &cells {
        for n in common::thread_counts() {
            g.bench_with_input(
                BenchmarkId::new(
                    "cell_analyse",
                    format!("{}/{}", cell.name[0], common::label(n)),
                ),
                &n,
                |b, &n| {
                    let p = common::pool(n);
                    common::in_pool(&p, || b.iter(|| black_box(cell.analyse().unwrap())));
                },
            );
        }
    }
    g.finish();
}

/// A single target over a whole run: cross-cell parallel `analyse`, then the arc, Verilog and Liberty
/// artifacts, each stated as values for all the cells at once and rendered once at its sink. Writing
/// the files and reporting the hazards are outside the measurement. Cross-cell and intra-cell
/// parallelism share the one installed pool.
fn bench_whole_run(c: &mut Criterion) {
    let mut g = c.benchmark_group("whole_run");
    let cells = common::raw_cells();
    for n in common::thread_counts() {
        g.bench_with_input(
            BenchmarkId::new("whole_run", common::label(n)),
            &n,
            |b, &n| {
                let p = common::pool(n);
                common::in_pool(&p, || {
                    b.iter(|| {
                        let analysed: Vec<_> =
                            cells.par_iter().map(|c| c.analyse().unwrap()).collect();
                        let rendered: Vec<CellArcs> = analysed
                            .par_iter()
                            .map(|c| cell_arcs(c, ArcsTclOptions::default()))
                            .collect();
                        let arcs = Deck(&rendered).to_string();
                        let declarations: Vec<_> =
                            analysed.par_iter().flat_map_iter(cell_verilog).collect();
                        let v = Verilog(&declarations).to_string();
                        let groups: Vec<_> =
                            analysed.par_iter().flat_map_iter(cell_liberty).collect();
                        let lib = library_liberty("cells", groups).to_string();
                        black_box((arcs, v, lib));
                    })
                });
            },
        );
    }
    g.finish();
}

criterion_group!(benches, bench_cell_analyse, bench_whole_run);
criterion_main!(benches);
