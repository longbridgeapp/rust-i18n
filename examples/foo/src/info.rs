use rust_i18n::t;

#[allow(unused)]
pub fn get_info() -> String {
    t!("hello").into()
}

#[cfg(test)]
mod tests {
    use super::get_info;

    #[test]
    fn test_get_info() {
        rust_i18n::set_locale("en");
        assert_eq!("Foo - Hello, World!", get_info());
        rust_i18n::set_locale("fr");
        assert_eq!("Foo - Bonjour, monde!", get_info());
    }
}
