use criterion::{criterion_group, criterion_main, Criterion};

use rlox::{Scanner};

pub fn criterion_benchmark(c: &mut Criterion) {
    let source = std::fs::read_to_string("./large.lox").unwrap();
    let scanner = Scanner::new(&source);

    let mut group = c.benchmark_group("rlox");

    group.bench_function("borrowing",
            |b| b.iter(|| scanner.tokens()));
    group.bench_function("owning",
        |b| b.iter(|| lox_syntax::parse(&source)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
