use rust_i18n::Backend;

rust_i18n::i18n!("../locales");

pub struct I18nBackend;

impl Backend for I18nBackend {
    fn available_locales(&self) -> Vec<&str> {
        _RUST_I18N_BACKEND.available_locales()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        let val = _RUST_I18N_BACKEND.translate(locale, key);
        if val.is_none() {
            _RUST_I18N_BACKEND.translate("en", key)
        } else {
            val
        }
    }
}
