use std::{path::PathBuf, sync::LazyLock, time::Duration};

use aoc_day::AoCDay;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day02::Day02;

static INPUT_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("inputs")
        .join("day02.txt")
});

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 02");

    group.sample_size(200);
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("count_safe_reports", move |bencher| {
        bencher.iter_batched(
            || {
                let mut day = Day02::default();
                day.load_input(&INPUT_PATH).unwrap();
                day
            },
            |day| day.count_safe_reports(),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("count_safe_reports2", move |bencher| {
        bencher.iter_batched(
            || {
                let mut day = Day02::default();
                day.load_input(&INPUT_PATH).unwrap();
                day
            },
            |day| day.count_safe_reports2(),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
