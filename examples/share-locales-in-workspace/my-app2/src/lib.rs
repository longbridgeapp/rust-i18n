rust_i18n::i18n!(backend = my_i18n::I18nBackend);

#[cfg(test)]
mod tests {
    use rust_i18n::t;

    #[test]
    fn test_load_str() {
        assert_eq!(
            t!("welcome", locale = "en"),
            "Rust I18n Example for share locales in entire workspace."
        );
        assert_eq!(
            t!("welcome", locale = "zh-CN"),
            "Rust I18n 示例，用于在整个工作区中共享本地化。"
        );
    }
}
