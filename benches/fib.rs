use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fib::{fib_rec, fib_linear};


fn fib_rec_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci-recursive");
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("fib_rec_20", |b| b.iter(|| black_box(fib_rec(20))));
    group.finish();
}

fn fib_linear_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci-linear");
    group.throughput(criterion::Throughput::Elements(1));
    group.bench_function("fib_linear_20", |b| b.iter(|| black_box(fib_linear(20))));
    group.finish();
}

criterion_group!(benches, fib_rec_benchmark, fib_linear_benchmark);
criterion_main!(benches);
