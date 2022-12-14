use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day14::{parse_input, part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");

    c.bench_function("parse", |b| {
        b.iter(|| black_box(parse_input(black_box(input))))
    });

    c.bench_function("part1", |b| {
        let walls = parse_input(input);
        b.iter(|| part1(black_box(&walls)))
    });

    c.bench_function("part2", |b| {
        let walls = parse_input(input);
        b.iter(|| part2(black_box(&walls)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
