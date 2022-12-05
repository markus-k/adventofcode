use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day05::move_crates;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("part1", |b| b.iter(|| move_crates(black_box(input), false)));
    c.bench_function("part2", |b| b.iter(|| move_crates(black_box(input), true)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
