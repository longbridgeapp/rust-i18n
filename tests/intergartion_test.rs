rust_i18n::i18n!("./tests/locales");

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    #[test]
    fn test_translate() {
        assert_eq!(crate::translate("en", "hello"), "Bar - Hello, World!");
    }

    #[test]
    fn test_available_locales() {
        assert_eq!(crate::available_locales(), &["en", "zh-CN"]);
    }

    #[test]
    fn it_foo_title() {
        rust_i18n::set_locale("en");
        assert_eq!(foo::t("hello"), "Foo - Hello, World!");
    }

    #[test]
    fn test_t() {
        rust_i18n::set_locale("en");
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

        rust_i18n::set_locale("zh-CN");
        assert_eq!(t!("messages.hello", name = "world"), "你好，world！");

        rust_i18n::set_locale("en");
        assert_eq!(t!("messages.hello", name = "world"), "Hello, world!");
    }

    #[test]
    fn test_t_with_tt_val() {
        rust_i18n::set_locale("en");

        assert_eq!(t!("messages.other", count = 100), "You have 100 messages.");
        assert_eq!(
            t!("messages.other", count = 1.01),
            "You have 1.01 messages."
        );
        assert_eq!(t!("messages.other", count = 1 + 2), "You have 3 messages.");

        // Test end with a comma
        assert_eq!(
            t!("messages.other", locale = "zh-CN", count = 1 + 2,),
            "你收到了 3 条新消息。"
        );

        let a = 100;
        assert_eq!(t!("messages.other", count = a / 2), "You have 50 messages.");
    }

    #[test]
    fn test_t_with_locale_and_args() {
        rust_i18n::set_locale("en");

        assert_eq!(t!("hello", locale = "zh-CN"), "Bar - 你好世界！");
        assert_eq!(t!("hello", locale = "en"), "Bar - Hello, World!");

        assert_eq!(t!("messages.hello", name = "Jason"), "Hello, Jason!");
        assert_eq!(
            t!("messages.hello", locale = "en", name = "Jason"),
            "Hello, Jason!"
        );
        // Invalid locale position, will ignore
        assert_eq!(
            t!("messages.hello", name = "Jason", locale = "en"),
            "Hello, Jason!"
        );
        assert_eq!(
            t!("messages.hello", locale = "zh-CN", name = "Jason"),
            "你好，Jason！"
        );
    }

    #[test]
    fn test_t_with_hash_args() {
        rust_i18n::set_locale("en");

        // Hash args
        assert_eq!(t!("messages.hello", name => "Jason"), "Hello, Jason!");
        assert_eq!(t!("messages.hello", "name" => "Jason"), "Hello, Jason!");
        assert_eq!(
            t!("messages.hello", locale = "zh-CN", "name" => "Jason"),
            "你好，Jason！"
        );
    }

    #[test]
    fn test_with_merge_file() {
        rust_i18n::set_locale("en");
        assert_eq!(t!("user.title"), "User Title");
        assert_eq!(t!("messages.user.title"), "Message User Title");
    }

    #[test]
    fn test_support_expr() {
        rust_i18n::set_locale("en");
        let name = "Jason Lee";
        let locale = "en";

        let key = "messages.hello";

        assert_eq!(
            t!(&format!("messages.{}", "hello"), name = name),
            "Hello, Jason Lee!"
        );
        assert_eq!(t!(key, name = name), "Hello, Jason Lee!");

        assert_eq!(t!("messages.hello", name = name), "Hello, Jason Lee!");
        assert_eq!(
            t!("messages.hello", name = &format!("this is {name}")),
            "Hello, this is Jason Lee!"
        );

        assert_eq!(t!("messages.hello", locale = locale), "Hello, %{name}!");

        assert_eq!(
            t!("messages.hello", name = name, locale = locale),
            "Hello, Jason Lee!"
        );
        assert_eq!(
            t!("messages.hello", name = name, locale = locale),
            "Hello, Jason Lee!"
        );
    }
}
