//! Whole-pipeline benchmarks: a single cell's full `analyse()` and the whole-crate run across all
//! example cells. Skeleton only for now — wave 2 wires each `bench_*` fn to its real target.

mod common;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_cell_analyse(c: &mut Criterion) {
    c.bench_function("_skeleton", |b| b.iter(|| common::raw_cells().len()));
}

fn bench_whole_run(c: &mut Criterion) {
    c.bench_function("_skeleton", |b| b.iter(|| common::raw_cells().len()));
}

criterion_group!(benches, bench_cell_analyse, bench_whole_run);
criterion_main!(benches);
