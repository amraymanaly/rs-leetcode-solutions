use criterion::{criterion_group, criterion_main, Criterion};
use solver::regular_expression_matching::is_match as func;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Regex trial", |b| {
        b.iter(|| func("aaaab".into(), "a*a*a*".into()))
    });
}

criterion_group! {name = benches; config = Criterion::default().significance_level(0.1).sample_size(10);
targets = criterion_benchmark}
criterion_main!(benches);
