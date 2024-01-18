#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use std::sync::Arc;

#[doc(hidden)]
pub use once_cell;
pub use rust_i18n_macro::i18n;
pub use rust_i18n_support::{AtomicStr, Backend, BackendExt, SimpleBackend};

static CURRENT_LOCALE: Lazy<AtomicStr> = Lazy::new(|| AtomicStr::from("en"));

/// Set current locale
pub fn set_locale(locale: &str) {
    CURRENT_LOCALE.replace(locale);
}

/// Get current locale
pub fn locale() -> Arc<String> {
    CURRENT_LOCALE.clone_string()
}

/// Get I18n text
///
/// ```no_run
/// #[macro_use] extern crate rust_i18n;
/// # fn _rust_i18n_translate(locale: &str, key: &str) -> String { todo!() }
/// # fn main() {
/// // Simple get text with current locale
/// t!("greeting"); // greeting: "Hello world" => "Hello world"
/// // Get a special locale's text
/// t!("greeting", locale = "de"); // greeting: "Hallo Welt!" => "Hallo Welt!"
///
/// // With variables
/// t!("messages.hello", name = "world"); // messages.hello: "Hello, {name}" => "Hello, world"
/// t!("messages.foo", name = "Foo", other ="Bar"); // messages.foo: "Hello, {name} and {other}" => "Hello, Foo and Bar"
///
/// // With locale and variables
/// t!("messages.hello", locale = "de", name = "Jason"); // messages.hello: "Hallo, {name}" => "Hallo, Jason"
/// # }
/// ```
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        crate::_rust_i18n_translate(rust_i18n::locale().as_str(), $key)
    };

    // t!("foo", locale = "en")
    ($key:expr, locale = $locale:expr) => {
        crate::_rust_i18n_translate($locale, $key)
    };

    // t!("foo", locale = "en", a = 1, b = "Foo")
    ($key:expr, locale = $locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            let mut message = crate::_rust_i18n_translate($locale, $key);

            $(
                // Get the variable name as a string, and remove quotes surrounding the variable name
                let var_name = stringify!($var_name).trim_matches('"');
                // Make a holder string to replace the variable name with: %{var_name}
                let holder = format!("%{{{var_name}}}");

                message = message.replace(&holder, &format!("{}", $var_val));
            )+
            message
        }
    };

    // t!("foo %{a} %{b}", a = "bar", b = "baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };
}

/// Get available locales
///
/// ```no_run
/// #[macro_use] extern crate rust_i18n;
/// # pub fn _rust_i18n_available_locales() -> Vec<&'static str> { todo!() }
/// # fn main() {
/// rust_i18n::available_locales!();
/// # }
/// // => ["en", "zh-CN"]
/// ```
#[macro_export(local_inner_macros)]
#[allow(clippy::crate_in_macro_def)]
macro_rules! available_locales {
    () => {
        crate::_rust_i18n_available_locales()
    };
}
