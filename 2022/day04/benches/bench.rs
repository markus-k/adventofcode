use day04::{find_fully_contained, find_overlapping};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("part1", |b| {
        b.iter(|| find_fully_contained(black_box(input)))
    });
    c.bench_function("part2", |b| b.iter(|| find_overlapping(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
