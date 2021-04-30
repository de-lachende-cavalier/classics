use std::fs;

use classical_cryptography::encrypt_data;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_monoalphabetic_encryption(c: &mut Criterion) {
    let data =
        fs::read_to_string("benches/shakespeare_henry5.txt").expect("Error reading Shakespeare.");
    let key = "zyxwvutsrqponmlkjihgfedcba";
    c.bench_function("monoalphabetic encrypt 'zyxwvutsrqponmlkjihgfedcba'", |b| {
        b.iter(|| encrypt_data("monoalphabetic", &data, key))
    });
}

criterion_group!(benches, benchmark_monoalphabetic_encryption);
criterion_main!(benches);
