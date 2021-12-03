//use aoc2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(_c: &mut Criterion) {
    /*
    {
        let input = std::fs::read_to_string("inputs/day24").unwrap();
        c.bench_function("24a", |b| b.iter(|| d24_lobby::part1(&input)));
        c.bench_function("24b", |b| b.iter(|| d24_lobby::part2(&input)));
    }
    */
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
