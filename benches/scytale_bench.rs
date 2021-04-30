use std::fs;

use classical_cryptography::encrypt_data;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_scytale_encryption(c: &mut Criterion) {
    let data =
        fs::read_to_string("benches/shakespeare_henry5.txt").expect("Error reading Shakespeare.");
    let key = "7";
    c.bench_function("scytale encrypt 7", |b| {
        b.iter(|| encrypt_data("scytale", &data, key))
    });
}

criterion_group!(benches, benchmark_scytale_encryption);
criterion_main!(benches);
