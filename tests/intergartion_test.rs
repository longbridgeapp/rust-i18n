#[macro_use]
extern crate rust_i18n;

i18n!("./tests/locales");

#[cfg(test)]
mod tests {
    #[test]
    fn it_foo_title() {
        assert_eq!(foo::t("hello"), "Foo - Hello, World!");
    }

    #[test]
    fn it_t() {
        assert_eq!(t!("hello"), "Bar - Hello, World!");

        // Vars
        assert_eq!(
            t!("a.very.nested.message"),
            "Hello, %{name}. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason"),
            "Hello, Jason. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason", msg = "Bla bla"),
            "Hello, Jason. Your message is: Bla bla"
        );

        rust_i18n::set_locale("de");
        assert_eq!(t!("messages.hello", name = "world"), "Hallo, world!");

        rust_i18n::set_locale("en");
        assert_eq!(t!("messages.hello", name = "world"), "Hello, world!");
    }

    #[test]
    fn it_t_with_locale_and_args() {
        assert_eq!(t!("hello", locale = "de"), "Bar - Hallo Welt!");
        assert_eq!(t!("hello", locale = "en"), "Bar - Hello, World!");

        rust_i18n::set_locale("en");
        assert_eq!(t!("messages.hello", name = "Jason"), "Hello, Jason!");
        assert_eq!(
            t!("messages.hello", locale = "en", name = "Jason"),
            "Hello, Jason!"
        );
        assert_eq!(
            t!("messages.hello", locale = "de", name = "Jason"),
            "Hallo, Jason!"
        );
    }

    #[test]
    fn it_with_merge_file() {
        assert_eq!(t!("user.title"), "User Title");
        assert_eq!(t!("messages.user.title"), "Message User Title");
    }
}
