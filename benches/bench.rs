use rust_i18n::t;

rust_i18n::i18n!("./tests/locales");

use criterion::{criterion_group, criterion_main, Criterion};

lazy_static::lazy_static! {
pub static ref DICT: std::collections::HashMap<&'static str, &'static str> =
    [
        ("hello", "Bar - Hello, World!"),
    ].iter().cloned().collect();
}

fn bench_t(c: &mut Criterion) {
    // 102 ns
    c.bench_function("t", |b| b.iter(|| t!("hello")));

    c.bench_function("t_with_locale", |b| b.iter(|| t!("hello", locale = "en")));

    // 73.239 ns
    c.bench_function("_rust_i18n_translate", |b| {
        b.iter(|| crate::_rust_i18n_translate("en", "hello"))
    });

    // 54.221 ns
    c.bench_function("_RUST_I18N_BACKEND.translate", |b| {
        b.iter(|| crate::_RUST_I18N_BACKEND.translate("en", "hello"))
    });

    // 46.721
    c.bench_function("static_hashmap_get_to_string", |b| {
        b.iter(|| DICT.get("hello").unwrap().to_string())
    });

    // 20.023 ns
    c.bench_function("static_hashmap_get_as_static_str", |b| {
        b.iter(|| DICT.get("hello").unwrap())
    });

    c.bench_function("t_with_args", |b| {
        b.iter(|| t!("a.very.nested.message", name = "Jason", msg = "Bla bla"))
    });

    c.bench_function("t_with_args (str)", |b| {
        b.iter(|| t!("a.very.nested.message", "name" = "Jason", "msg" = "Bla bla"))
    });
}

criterion_group!(benches, bench_t);
criterion_main!(benches);
