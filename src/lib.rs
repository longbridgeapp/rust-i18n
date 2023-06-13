#![allow(rustdoc::invalid_rust_codeblocks)]
#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[doc(hidden)]
pub use once_cell;
pub use rust_i18n_macro::i18n;

static CURRENT_LOCALE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("en")));

/// Set current locale
pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = locale.to_string();
}

/// Get current locale
pub fn locale() -> String {
    CURRENT_LOCALE.lock().unwrap().to_string()
}

/// Get I18n text
///
/// ```ignore
/// // Simple get text with current locale
/// t!("greeting"); // greeting: "Hello world" => "Hello world"
/// // Get a special locale's text
/// t!("greeting", locale = "de"); // greeting: "Hallo Welt!" => "Hallo Welt!"
///
/// // With variables
/// t!("messages.hello", "world"); // messages.hello: "Hello, {}" => "Hello, world"
/// t!("messages.foo", "Foo", "Bar"); // messages.foo: "Hello, {} and {}" => "Hello, Foo and Bar"
///
/// // With locale and variables
/// t!("messages.hello", locale = "de", "Jason"); // messages.hello: "Hallo, {}" => "Hallo, Jason"
/// ```
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        crate::_rust_i18n_translate(rust_i18n::locale().as_str(), $key)
    };

    // t!("foo", locale="en")
    ($key:expr, locale=$locale:expr) => {
        crate::_rust_i18n_translate($locale, $key)
    };

    // t!("foo", locale="en", a=1, b="Foo")
    ($key:expr, locale=$locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            let mut message = crate::_rust_i18n_translate($locale, $key);
            $(
                let var = stringify!($var_name).trim_matches('"');
                let mut holder = std::string::String::from("%{");
                holder.push_str(var);
                holder.push('}');

                message = message.replace(&holder, &format!("{}", $var_val));
            )+
            message
        }
    };

    // t!("foo %{a} %{b}", a="bar", b="baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:expr => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:expr => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! map {
    {$($key:expr => $value:expr),+} => {{
        let mut m = std::collections::HashMap::new();
        $(
            m.insert($key, $value);
        )+
        m
    }};
}
