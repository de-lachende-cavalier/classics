use std::fs;

use classical_cryptography::encrypt_data;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_vigenere_encryption(c: &mut Criterion) {
    let data =
        fs::read_to_string("benches/shakespeare_henry5.txt").expect("Error reading Shakespeare.");
    let key = "rebus";
    c.bench_function("vigenere encrypt 'rebus'", |b| {
        b.iter(|| encrypt_data("vigenere", &data, key))
    });
}

criterion_group!(benches, benchmark_vigenere_encryption);
criterion_main!(benches);
