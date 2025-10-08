use criterion::{criterion_group, criterion_main, Criterion};
use csd::csd::{to_csd, to_csd_i};

fn csd_benchmark(c: &mut Criterion) {
    c.bench_function("to_csd", |b| {
        b.iter(|| to_csd(std::hint::black_box(28.5), std::hint::black_box(10)))
    });

    c.bench_function("to_csd_i", |b| {
        b.iter(|| to_csd_i(std::hint::black_box(28)))
    });
}

criterion_group!(benches, csd_benchmark);
criterion_main!(benches);
