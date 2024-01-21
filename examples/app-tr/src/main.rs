use rust_i18n::tr;

rust_i18n::i18n!("locales");

#[cfg(feature = "log-tr-dyn")]
fn set_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .parse_default_env()
        .init();
}

#[cfg(not(feature = "log-tr-dyn"))]
fn set_logger() {}

fn main() {
    set_logger();

    let locales = rust_i18n::available_locales!();
    println!("Available locales: {:?}", locales);
    println!();

    println!("String literals with patterns translation:");
    for locale in &locales {
        println!(
            "Hello, %{{name}}! => {} ({})",
            tr!("Hello, %{name}!", name = "World", locale = locale),
            locale
        );
    }
    println!();

    println!("String literals translation:");
    for locale in &locales {
        println!(
            "{:>8} => {} ({})",
            "Hello",
            tr!("Hello", locale = locale),
            locale
        );
    }
    println!();

    // Identify the locale message by number.
    // For example, the `tr!` will find the translation with key named "tr_N_5" for 5.
    println!("Numeric literals translation:");
    for locale in &locales {
        println!("{:>8} => {} ({})", 5, tr!(5, locale = locale), locale);
    }
    println!();

    println!("Missing translations:");
    for locale in &locales {
        println!(
            "{:>8} => {} ({locale})",
            "The message is untranslated!",
            tr!("The message is untranslated!", locale = locale)
        );
    }
    println!();

    println!("Runtime string translation:");
    let src_list = ["Apple", "Banana", "Orange"];
    for src in src_list.iter() {
        for locale in &locales {
            let translated = tr!(*src, locale = locale);
            println!("{:>8} => {} ({locale})", src, translated);
        }
    }
    println!();

    println!("Runtime numeric translation:");
    for i in 0..10usize {
        for locale in &locales {
            println!("{:>8} => {} ({locale})", i, tr!(i, locale = locale));
        }
    }
    println!();
}
