use rust_i18n::t;

rust_i18n::i18n!(
    "locales",
    minify_key = true,
    minify_key_len = 24,
    minify_key_prefix = "T.",
    minify_key_thresh = 4
);

#[cfg(feature = "log-missing")]
fn set_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .parse_default_env()
        .init();
}

#[cfg(not(feature = "log-missing"))]
fn set_logger() {}

fn main() {
    set_logger();

    let locales = rust_i18n::available_locales!();
    println!("Available locales: {:?}", locales);
    println!();

    println!("Translation of string literals:");
    for locale in &locales {
        println!(
            "{:>8} => {} ({})",
            "Hello",
            t!("Hello", locale = locale),
            locale
        );
    }
    println!();

    println!("Translation of string literals with patterns:");
    for locale in &locales {
        println!(
            "Hello, %{{name}}! => {} ({})",
            t!("Hello, %{name}!", name = "World", locale = locale),
            locale
        );
    }
    println!();

    println!("Translation of string literals with specified arguments:");
    for i in (0..10000).step_by(50) {
        println!(
            "Zero padded number: %{{count}} => {}",
            t!("Zero padded number: %{count}", count = i : {:08}),
        );
    }
    println!();

    println!("Handling of missing translations:");
    for locale in &locales {
        println!(
            "{:>8} => {} ({locale})",
            "The message is untranslated!",
            t!("The message is untranslated!", locale = locale)
        );
    }
    println!();

    println!("Translation of runtime strings:");
    let src_list = ["Apple", "Banana", "Orange"];
    for src in src_list.iter() {
        for locale in &locales {
            let translated = t!(*src, locale = locale);
            println!("{:>8} => {} ({locale})", src, translated);
        }
    }
    println!();

    if cfg!(feature = "log-missing") {
        println!("Translates runtime strings and logs when a lookup is missing:");
        for locale in &locales {
            let msg = "Foo Bar".to_string();
            println!("{:>8} => {} ({locale})", &msg, t!(&msg, locale = locale));
        }
        println!();
    }
}
