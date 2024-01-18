#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use std::sync::Arc;

#[doc(hidden)]
pub use once_cell;
pub use rust_i18n_macro::{i18n, key};
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

/// Replace patterns and return a new string.
///
/// # Arguments
///
/// * `input` - The input string, containing patterns like `%{name}`.
/// * `patterns` - The patterns to replace.
/// * `values` - The values to replace.
///
/// # Example
///
/// ```
/// # use rust_i18n::replace_patterns;
/// let input = "Hello, %{name}!";
/// let patterns = &["name"];
/// let values = &["world".to_string()];
/// let output = replace_patterns(input, patterns, values);
/// assert_eq!(output, "Hello, world!");
/// ```
pub fn replace_patterns(input: &str, patterns: &[&str], values: &[String]) -> String {
    let input_bytes = input.as_bytes();
    let mut pattern_pos = smallvec::SmallVec::<[usize; 64]>::new();
    let mut stage = 0;
    for (i, &b) in input_bytes.iter().enumerate() {
        match (stage, b) {
            (1, b'{') => {
                stage = 2;
                pattern_pos.push(i);
            }
            (2, b'}') => {
                stage = 0;
                pattern_pos.push(i);
            }
            (_, b'%') => {
                stage = 1;
            }
            _ => {}
        }
    }
    let mut output: Vec<u8> = Vec::with_capacity(input_bytes.len() + 128);
    let mut prev_end = 0;
    let pattern_values = patterns.iter().zip(values.iter());
    for pos in pattern_pos.chunks_exact(2) {
        let start = pos[0];
        let end = pos[1];
        let key = &input_bytes[start + 1..end];
        if prev_end < start {
            let prev_chunk = &input_bytes[prev_end..start - 1];
            output.extend_from_slice(prev_chunk);
        }
        if let Some((_, v)) = pattern_values
            .clone()
            .find(|(&pattern, _)| pattern.as_bytes() == key)
        {
            output.extend_from_slice(v.as_bytes());
        } else {
            output.extend_from_slice(&input_bytes[start - 1..end + 1]);
        }
        prev_end = end + 1;
    }
    if prev_end < input_bytes.len() {
        let remaining = &input_bytes[prev_end..];
        output.extend_from_slice(remaining);
    }
    unsafe { String::from_utf8_unchecked(output) }
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
            let message = crate::_rust_i18n_translate($locale, $key);
            let patterns: &[&str] = &[
                $(rust_i18n::key!($var_name)),+
            ];
            let values = &[
                $(format!("{}", $var_val)),+
            ];

            rust_i18n::replace_patterns(message.as_ref(), patterns, values)
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
