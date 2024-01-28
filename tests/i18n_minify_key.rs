rust_i18n::i18n!(
    "./tests/locales",
    fallback = "en",
    minify_key = true,
    minify_key_len = 24,
    minify_key_prefix = "tr_",
    minify_key_thresh = 4
);

#[cfg(test)]
mod tests {
    use super::*;
    use rust_i18n::{t, tkv};

    #[test]
    fn test_i18n_attrs() {
        assert_eq!(crate::_RUST_I18N_MINIFY_KEY, true);
        assert_eq!(crate::_RUST_I18N_MINIFY_KEY_LEN, 24);
        assert_eq!(crate::_RUST_I18N_MINIFY_KEY_PREFIX, "tr_");
        assert_eq!(crate::_RUST_I18N_MINIFY_KEY_THRESH, 4);
    }

    #[test]
    fn test_t() {
        assert_eq!(t!("Bar - Hello, World!"), "Bar - Hello, World!");
        assert_eq!(
            t!("Bar - Hello, World!", locale = "en"),
            "Bar - Hello, World!"
        );
        assert_eq!(
            t!("Bar - Hello, World!", locale = "zh-CN"),
            "Bar - 你好世界！"
        );
        let fruits = vec!["Apple", "Banana", "Orange"];
        let fruits_translated = vec!["苹果", "香蕉", "橘子"];
        for (src, dst) in fruits.iter().zip(fruits_translated.iter()) {
            assert_eq!(t!(*src, locale = "zh-CN"), *dst);
        }
        let msg = "aka".to_string();
        let i = 0;
        assert_eq!(t!(msg, name => & i : {} ), "aka");
        assert_eq!(t!("hello"), "hello");
        assert_eq!(t!("hello",), "hello");
        assert_eq!(t!("hello", locale = "en"), "hello");
        assert_eq!(t!(format!("hello"), locale = "en"), "hello");
        assert_eq!(t!("Hello, %{name}", name = "Bar"), "Hello, Bar");
        assert_eq!(
            t!("You have %{count} messages.", locale = "zh-CN", count = 1 + 2,,,),
            "你收到了 3 条新消息。"
        );
        let (key, msg) = tkv!(
            r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque sed nisi leo. Donec commodo in ex at aliquam. Nunc in aliquam arcu. Fusce mollis metus orci, ut sagittis erat lobortis sed. Morbi quis arcu ultrices turpis finibus tincidunt non in purus. Donec gravida condimentum sapien. Duis iaculis fermentum congue. Quisque blandit libero a lacus auctor vestibulum. Nunc efficitur sollicitudin nisi, sit amet tristique lectus mollis non. Praesent sit amet erat volutpat, pharetra orci eget, rutrum felis. Sed elit augue, imperdiet eu facilisis vel, finibus vel urna. Duis quis neque metus.

            Mauris suscipit bibendum mattis. Vestibulum eu augue diam. Morbi dapibus tempus viverra. Sed aliquam turpis eget justo ornare maximus vitae et tortor. Donec semper neque sit amet sapien congue scelerisque. Maecenas bibendum imperdiet dolor interdum facilisis. Integer non diam tempus, pharetra ex at, euismod diam. Ut enim turpis, sagittis in iaculis ut, finibus et sem. Suspendisse a felis euismod neque euismod placerat. Praesent ipsum libero, porta vel egestas quis, aliquet vitae lorem. Nullam vel pharetra erat, sit amet sodales leo."#
        );
        assert_eq!(t!((key, msg)).as_ptr(), msg.as_ptr());
        assert_eq!(t!((key, msg), locale = "en"), msg);
        assert_eq!(t!((key, msg), locale = "de"), msg);
        assert_eq!(t!((key, msg), locale = "zh"), msg);
    }

    #[test]
    fn test_tkv() {
        let (key, msg) = tkv!("");
        assert_eq!(key, "");
        assert_eq!(msg, "");
        let (key, msg) = tkv!("Hey");
        assert_eq!(key, "Hey");
        assert_eq!(msg, "Hey");
        let (key, msg) = tkv!("Hello, world!");
        assert_eq!(key, "tr_1LokVzuiIrh1xByyZG4wjZ");
        assert_eq!(msg, "Hello, world!");
    }
}
