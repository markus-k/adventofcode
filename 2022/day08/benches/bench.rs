use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day08::{best_score, parse_input, visible_trees};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("parse", |b| b.iter(|| black_box(parse_input(input))));

    c.bench_function("part1", |b| {
        let map = parse_input(input);

        b.iter(|| visible_trees(black_box(&map)))
    });
    c.bench_function("part2", |b| {
        let map = parse_input(input);

        b.iter(|| best_score(black_box(&map)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
