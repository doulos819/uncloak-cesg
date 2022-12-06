
#![allow(unused)]
fn main() {
use vignere::vignere::Vignere
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vignere::vignere;

pub fn vig_bench_en(c: &mut Criterion) {
    let vignere = vignere::new();

    c.bench_function("encrypt", |f| f.iter(|| vignere.encrypt(&self, b"some_text")));
}

pub fn vig_bench_de(c: &mut Criterion) {
    let vignere = Vignere::new();
    let ciphertext: Vec<u8> = vignere.encrypt(b"some_text");

    c.bench_function("decrypt", |f| f.iter(|| vignere.decrypt(&ciphertext)));
}

criterion_group!(benches, vignere_benchmark);
criterion_main!(benches);
}
