//! Per-stage benchmarks: signal-BDD build, machine build/derive/detect/leakage, and emit. Skeleton
//! only for now — wave 2 wires each `bench_*` fn to its real stage.

mod common;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_signal_stages(c: &mut Criterion) {
    c.bench_function("_skeleton", |b| b.iter(|| common::raw_cells().len()));
}

fn bench_machine_stages(c: &mut Criterion) {
    c.bench_function("_skeleton", |b| b.iter(|| common::raw_cells().len()));
}

fn bench_emit_stages(c: &mut Criterion) {
    c.bench_function("_skeleton", |b| b.iter(|| common::raw_cells().len()));
}

criterion_group!(
    benches,
    bench_signal_stages,
    bench_machine_stages,
    bench_emit_stages
);
criterion_main!(benches);
