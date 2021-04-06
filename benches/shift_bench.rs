use std::fs;

use classical_cryptography::encrypt_data;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_shift_encryption(c: &mut Criterion) {
    let data =
        fs::read_to_string("benches/shakespeare_henry5.txt").expect("Error reading Shakespeare.");
    c.bench_function("shift encrypt 20", |b| {
        b.iter(|| encrypt_data("shift", &data, "20"))
    });
}

criterion_group!(benches, benchmark_shift_encryption);
criterion_main!(benches);
