use rust_i18n::t;

mod info;

rust_i18n::i18n!("locales");

pub fn t(key: &str) -> String {
    t!(key)
}
