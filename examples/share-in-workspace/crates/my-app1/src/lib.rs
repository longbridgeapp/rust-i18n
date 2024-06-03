use i18n::t;

i18n::init!();

#[allow(dead_code)]
fn assert_messages() {
    assert_eq!(
        t!("welcome", locale = "en"),
        "Rust I18n Example for share locales in entire workspace."
    );
    assert_eq!(
        t!("welcome", locale = "zh-CN"),
        "Rust I18n 示例，用于在整个工作区中共享本地化。"
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_str() {
        super::assert_messages();
    }
}
