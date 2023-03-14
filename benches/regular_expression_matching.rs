use criterion::{criterion_group, criterion_main, Criterion};
use solver::regular_expression_matching::is_match as func;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Regex trial", |b| {
        b.iter(|| func("b".into(), "b*b*ba*".into()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
