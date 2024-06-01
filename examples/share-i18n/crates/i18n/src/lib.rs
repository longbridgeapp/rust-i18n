use rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

pub struct Backend;

impl rust_i18n::Backend for Backend {
    fn available_locales(&self) -> Vec<&str> {
        _RUST_I18N_BACKEND.available_locales()
    }

    fn translate<'a>(&'a self, locale: &str, key: &str) -> Option<&str> {
        _RUST_I18N_BACKEND.translate(locale, key)
    }
}
