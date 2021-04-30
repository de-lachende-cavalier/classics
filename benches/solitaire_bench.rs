use std::fs;

use classical_cryptography::encrypt_data;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_solitaire_encryption(c: &mut Criterion) {
    let data =
        fs::read_to_string("benches/shakespeare_henry5.txt").expect("Error reading Shakespeare.");
    let key = "cards upon cards";
    c.bench_function("shift encrypt 'cards upon cards'", |b| {
        b.iter(|| encrypt_data("solitaire", &data, key))
    });
}

criterion_group!(benches, benchmark_solitaire_encryption);
criterion_main!(benches);
