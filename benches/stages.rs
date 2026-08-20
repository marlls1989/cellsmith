//! Per-stage benchmarks: signal-BDD build, machine build/derive/detect/leakage, and emit. Every
//! target is a pipeline stage keyed `BenchmarkId(stage, "{cell}/{width}")` and measured at each of
//! [`common::thread_counts`]. A pinned width gets a scoped rayon pool built once per registration
//! (outside `b.iter`), never a process-global one; unpinned runs on the global pool as the tool does.

mod common;

use cellsmith::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use cellsmith::emit::liberty::cell_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::logic::analysis::{analyse_machine, Machine};
use cellsmith::logic::machine::ExplorationBudget;
use cellsmith::logic::minimise::{minimise_state_space, Preserved};
use cellsmith::logic::{arcs, confluence, leakage, width};
use cellsmith::model::{build_signal_bdds, derive_regions};
use espresso_logic::{sync_bdd_builder, Symbol};
use liberty_parse::liberty::Liberty;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

/// Register one stage target at every measured thread count, in the uniform bench shape: build the
/// pool once per registration (outside `b.iter`) and run the timed `$routine` inside it.
macro_rules! sweep_bench {
    ($group:expr, $stage:expr, $name:expr, $routine:expr $(,)?) => {
        for n in common::thread_counts() {
            $group.bench_with_input(
                BenchmarkId::new($stage, format!("{}/{}", $name, common::label(n))),
                &n,
                |b, &n| {
                    let p = common::pool(n);
                    common::in_pool(&p, || b.iter($routine));
                },
            );
        }
    };
}

fn bench_signal_stages(c: &mut Criterion) {
    let mut g = c.benchmark_group("signal");
    for cell in common::raw_cells() {
        // Pre-minimise fixture, plus the signal order and preserved set the minimise pass needs.
        let pre = cell.analyse_signals().unwrap();
        let order: Vec<Symbol> = pre.signals().map(|s| s.name.clone()).collect();
        let preserved = Preserved::outputs(pre.outputs.iter().map(|o| o.name.clone()).collect());

        // Re-parse and re-classify the cell's signals each iteration.
        sweep_bench!(g, "parse", cell.name[0], || cell.analyse_signals().unwrap());

        // Mint the per-cell builder inside the timed closure so the BDD memo does not warm across
        // iterations.
        sweep_bench!(g, "build_signal_bdds", cell.name[0], || {
            let builder = sync_bdd_builder!();
            build_signal_bdds(&pre, &builder)
        });

        // A fresh signal-BDD map per iteration (minted and dropped in setup — the Bdd handles survive
        // the builder drop), since minimise_state_space rewrites the map in place.
        for n in common::thread_counts() {
            g.bench_with_input(
                BenchmarkId::new("minimise", format!("{}/{}", cell.name[0], common::label(n))),
                &n,
                |b, &n| {
                    let p = common::pool(n);
                    common::in_pool(&p, || {
                        b.iter_batched(
                            || {
                                let builder = sync_bdd_builder!();
                                build_signal_bdds(&pre, &builder)
                            },
                            |mut m| minimise_state_space(&mut m, &order, &preserved),
                            BatchSize::SmallInput,
                        )
                    });
                },
            );
        }
    }
    g.finish();
}

fn bench_machine_stages(c: &mut Criterion) {
    let mut g = c.benchmark_group("machine");
    for cell in common::raw_cells() {
        // Fixture built once per cell: analyse folds the exprs post-minimise, so this map equals the
        // minimised map Machine::build consumes. The else-continue skips a cell whose exploration
        // passes an ExplorationBudget ceiling — there is no machine to time.
        let ac = cell.analyse().unwrap();
        let builder = sync_bdd_builder!();
        let bdds = build_signal_bdds(&ac, &builder);
        let budget = ExplorationBudget::default();
        let Ok(m) = Machine::build(
            &ac,
            &bdds,
            cellsmith::logic::analysis::Exploration::Fresh(&budget),
        ) else {
            continue;
        };

        sweep_bench!(g, "machine_build", cell.name[0], || {
            Machine::build(
                &ac,
                &bdds,
                cellsmith::logic::analysis::Exploration::Fresh(&budget),
            )
            .unwrap()
        });
        sweep_bench!(g, "arcs_derive", cell.name[0], || arcs::derive(&m));
        sweep_bench!(g, "confluence_detect", cell.name[0], || {
            confluence::detect(&m)
        });
        sweep_bench!(g, "width_detect", cell.name[0], || { width::detect(&m) });
        sweep_bench!(g, "analyse_machine", cell.name[0], || {
            analyse_machine(
                &ac,
                &bdds,
                true,
                cellsmith::logic::analysis::Exploration::Fresh(&budget),
            )
        });
        sweep_bench!(g, "leakage_derive", cell.name[0], || {
            leakage::derive(&m)
        });
        sweep_bench!(g, "derive_regions", cell.name[0], || {
            derive_regions(&ac, &bdds)
        });
    }
    g.finish();
}

fn bench_emit_stages(c: &mut Criterion) {
    let mut g = c.benchmark_group("emit");
    for cell in common::raw_cells() {
        let ac = cell.analyse().unwrap();

        sweep_bench!(g, "cell_arcs_tcl", cell.name[0], || {
            cell_arcs_tcl(&ac, ArcsTclOptions::default())
        });
        sweep_bench!(g, "cell_verilog", cell.name[0], || { cell_verilog(&ac) });
        sweep_bench!(g, "cell_liberty", cell.name[0], || {
            Liberty(cell_liberty(&ac)).to_string()
        });
    }
    g.finish();
}

criterion_group!(
    benches,
    bench_signal_stages,
    bench_machine_stages,
    bench_emit_stages
);
criterion_main!(benches);
