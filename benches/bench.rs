use rust_i18n::t;

rust_i18n::i18n!("./tests/locales");

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_t(c: &mut Criterion) {
    c.bench_function("t", |b| b.iter(|| t!("hello")));

    c.bench_function("t_with_args", |b| {
        b.iter(|| t!("a.very.nested.message", name = "Jason", msg = "Bla bla"))
    });

    c.bench_function("t_with_args (str)", |b| {
        b.iter(|| t!("a.very.nested.message", "name" = "Jason", "msg" = "Bla bla"))
    });
}

criterion_group!(benches, bench_t);
criterion_main!(benches);
