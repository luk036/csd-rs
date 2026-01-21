use criterion::{criterion_group, criterion_main, Criterion};
use csd::{
    csd::{to_csd, to_csd_i, to_csdnnz, to_csdnnz_i, to_decimal, to_decimal_i},
    lcsre::longest_repeated_substring,
};

fn csd_benchmark(c: &mut Criterion) {
    // Basic conversion functions
    c.bench_function("to_csd", |b| {
        b.iter(|| to_csd(std::hint::black_box(28.5), std::hint::black_box(10)))
    });

    c.bench_function("to_csd_i", |b| {
        b.iter(|| to_csd_i(std::hint::black_box(28)))
    });

    // Non-zero limited conversions
    c.bench_function("to_csdnnz", |b| {
        b.iter(|| to_csdnnz(std::hint::black_box(28.5), std::hint::black_box(4)))
    });

    c.bench_function("to_csdnnz_i", |b| {
        b.iter(|| to_csdnnz_i(std::hint::black_box(28), std::hint::black_box(4)))
    });

    // Reverse conversions
    c.bench_function("to_decimal", |b| {
        b.iter(|| to_decimal(std::hint::black_box("+00-00.+0")))
    });

    c.bench_function("to_decimal_i", |b| {
        b.iter(|| to_decimal_i(std::hint::black_box("+00-00")))
    });

    // Edge cases
    c.bench_function("to_csd_zero", |b| {
        b.iter(|| to_csd(std::hint::black_box(0.0), std::hint::black_box(10)))
    });

    c.bench_function("to_csd_negative", |b| {
        b.iter(|| to_csd(std::hint::black_box(-28.5), std::hint::black_box(10)))
    });

    c.bench_function("to_csd_small", |b| {
        b.iter(|| to_csd(std::hint::black_box(0.5), std::hint::black_box(10)))
    });

    c.bench_function("to_csd_large", |b| {
        b.iter(|| to_csd(std::hint::black_box(1024.75), std::hint::black_box(10)))
    });

    // Longest repeated substring
    c.bench_function("longest_repeated_substring", |b| {
        b.iter(|| longest_repeated_substring(std::hint::black_box("+-00+-00+-00+-0")))
    });

    c.bench_function("longest_repeated_substring_no_repeat", |b| {
        b.iter(|| longest_repeated_substring(std::hint::black_box("abcdefgh")))
    });

    c.bench_function("longest_repeated_substring_long", |b| {
        b.iter(|| {
            longest_repeated_substring(std::hint::black_box("abcabcabcabcabcabcabcabcabcabc"))
        })
    });
}

criterion_group!(benches, csd_benchmark);
criterion_main!(benches);
