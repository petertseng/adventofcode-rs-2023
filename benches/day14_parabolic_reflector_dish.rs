use criterion::{black_box, criterion_group, criterion_main, Criterion};

use adventofcode::day14_parabolic_reflector_dish::scan;
use adventofcode::day14_parabolic_reflector_dish::sort_rocks_naive;
use adventofcode::day14_parabolic_reflector_dish::sort_rocks_skip;
use adventofcode::day14_parabolic_reflector_dish::u128_row;
#[cfg(feature = "bigint")]
use adventofcode::day14_parabolic_reflector_dish::bigint;

fn criterion_benchmark(c: &mut Criterion) {
    let s = std::fs::read_to_string("../adventofcode-common/secret-cases/2023/14p.in")
        .expect("file should be present at the standard path");
    {
        let mut group = c.benchmark_group("parse platform");
        group.bench_function("scan", |b| b.iter(|| scan::platform(black_box(&s))));
        group.bench_function("sort_rocks_naive", |b| b.iter(|| sort_rocks_naive::platform(black_box(&s))));
        group.bench_function("sort_rocks_skip", |b| b.iter(|| sort_rocks_skip::platform(black_box(&s))));
        group.bench_function("u128_row", |b| b.iter(|| u128_row::platform(black_box(&s))));
        #[cfg(feature = "bigint")]
        group.bench_function("bigint", |b| b.iter(|| bigint::platform(black_box(&s))));
    }

    {
        let mut group = c.benchmark_group("parse + loads");
        group.sample_size(10);
        group.bench_function("scan", |b| b.iter(|| scan::loads(scan::platform(black_box(&s)))));
        group.bench_function("sort_rocks_naive", |b| b.iter(|| sort_rocks_naive::loads(sort_rocks_naive::platform(black_box(&s)))));
        group.bench_function("sort_rocks_skip", |b| b.iter(|| sort_rocks_skip::loads(sort_rocks_skip::platform(black_box(&s)))));
        group.bench_function("u128_row", |b| b.iter(|| u128_row::loads(u128_row::platform(black_box(&s)))));
        #[cfg(feature = "bigint")]
        group.bench_function("bigint", |b| b.iter(|| bigint::loads(bigint::platform(black_box(&s)))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
