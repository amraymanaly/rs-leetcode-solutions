use criterion::{criterion_group, criterion_main, Criterion};
use solver::longest_substring_without_repeating_characters::length_of_longest_substring as func;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("LSWRC example3", |b| b.iter(|| func("z,jsdfzkhsdflkjshdfsjkhsdkzxcvbnm,asdkfjgh;qpwoeirutyjfahlskjdhfkjasdghjbasdfsbjhbfkjbshekrgbwriurwtjuoirytwue".into())));

    c.bench_function("LSWRC lorem ipsum", |b| b.iter(|| func("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
