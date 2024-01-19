#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");

pub fn hello(name: &str) -> String {
    t!("hello", name = name).into()
}
