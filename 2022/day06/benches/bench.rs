use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day06::find_start_marker;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("part1", |b| {
        b.iter(|| find_start_marker(black_box(input), 4))
    });
    c.bench_function("part2", |b| {
        b.iter(|| find_start_marker(black_box(input), 14))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
