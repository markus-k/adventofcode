use day03::{elve_groups, find_common_priority};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("part1", |b| {
        b.iter(|| find_common_priority(black_box(input)))
    });
    c.bench_function("part2", |b| b.iter(|| elve_groups(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
