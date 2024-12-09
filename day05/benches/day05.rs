use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

const INPUT: &str = include_str!("../example_input.txt");

fn input_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("input_into_updates", |b| {
        b.iter(|| {
            black_box(day05::parse::input_to_updates(INPUT));
        })
    });
}

criterion_group!(benches, input_parsing_benchmark);
criterion_main!(benches);
