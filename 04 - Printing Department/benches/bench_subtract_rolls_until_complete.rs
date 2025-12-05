use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_subtract_rolls_until_complete(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").expect("failed to read from file");
    let rolls = input.parse::<printing_department::CellSet>().expect("failed to parse input");
    c.bench_function("bench_subtract_rolls_until_complete", |b| {
        b.iter(|| printing_department::subtract_rolls_until_complete(black_box(rolls.clone())))
    });
}

criterion_group!(benches, bench_subtract_rolls_until_complete);

criterion_main!(benches);
