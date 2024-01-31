use rust_i18n::t;

rust_i18n::i18n!();

fn main() {
    let locales = rust_i18n::available_locales!();
    println!("Available locales: {:?}", locales);
    println!();

    assert_eq!(t!("a"), "甲");
    assert_eq!(t!("ab"), "甲乙");
    assert_eq!(t!("abc"), "甲乙丙");
    assert_eq!(t!("abcd"), "甲乙丙丁");
    assert_eq!(t!("abcde"), "甲乙丙丁戊");
    assert_eq!(t!("abcdef"), "甲乙丙丁戊己");
    assert_eq!(t!("Hello, world!"), "你好，世界！");
    assert_eq!(t!("a", locale = "en"), "A");
    assert_eq!(t!("ab", locale = "en"), "AB");
    assert_eq!(t!("abc", locale = "en"), "ABC");
    assert_eq!(t!("abcd", locale = "en"), "ABCD");
    assert_eq!(t!("abcde", locale = "en"), "ABCDE");
    assert_eq!(t!("abcdef", locale = "en"), "ABCDEF");
    assert_eq!(t!("Hello, world!", locale = "en"), "Hello, world!");
}
