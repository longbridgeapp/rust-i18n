# An example of share I18n translations between crates.

This example is shows how to share I18n translations between crates. We only embed the translations into `i18n` crate once, and then share them with other crates.

Unlike use `rust_i18n::i18n!` macro to load translations in each crate, we just only load translations in `i18n` crate, it will save memory and reduce the size of the final binary.

本示例展示了如何在不同的 crate 之间共享 I18n 翻译。我们只需要在 `i18n` crate 中嵌入翻译一次，然后就可以在其他 crate 中共享它们。

与在每个 crate 中使用 `rust_i18n::i18n!` 宏加载翻译不同，我们只需要在 `i18n` crate 中加载翻译，这样可以节省内存并减小最终二进制文件的大小。

- [i18n](crate/i18n) - Used to load and share translations, and provide a macro to get translations.
- [my-app1](crate/my-app1) - An example of using `i18n` crate to get translations.
- [my-app2](crate/my-app2) - Another example of using `i18n` crate to get translations.
