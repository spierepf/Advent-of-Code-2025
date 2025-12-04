use criterion::{criterion_main, criterion_group, Criterion};
use std::hint::black_box;

fn bench_max_joltage(c: &mut Criterion) {
    let bank = "2555245573282137352766682525526364435746545343523394355638332326665366122245646523573255525564158774";
    let mut g = c.benchmark_group("bench_max_joltage");
    for connection_count in 1..=12 {
        g.throughput(criterion::Throughput::Elements(connection_count as u64));
        g.bench_with_input(criterion::BenchmarkId::new("max_joltage", &connection_count), &connection_count, |b, &connection_count| b.iter(|| lobby::max_joltage(black_box(bank), black_box(connection_count))));
        g.bench_with_input(criterion::BenchmarkId::new("max_joltage_dp", &connection_count), &connection_count, |b, &connection_count| b.iter(|| lobby::max_joltage_dp(black_box(bank), black_box(connection_count))));
    }
    g.finish();
}

criterion_group!(benches, bench_max_joltage);

criterion_main!(benches);