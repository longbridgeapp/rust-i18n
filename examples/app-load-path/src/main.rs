#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("../locales");

fn hello() -> String {
    t!("hello")
}

fn main() {
    rust_i18n::set_locale("en");
    assert_eq!("examples: Hello", hello());
    rust_i18n::set_locale("fr");
    assert_eq!("examples: Bonjour", hello());
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_hello() {
        rust_i18n::set_locale("en");
        assert_eq!("examples: Hello", hello());
        rust_i18n::set_locale("fr");
        assert_eq!("examples: Bonjour", hello());
    }
}
