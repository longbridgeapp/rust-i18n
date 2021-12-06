#[macro_use]
extern crate rust_i18n;

i18n!("./locales");

pub fn t(key: &str) -> String {
    t!(key)
}
