/*!
[![CI](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml/badge.svg)](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml) [![Docs](https://docs.rs/rust-i18n/badge.svg)](https://docs.rs/rust-i18n/) [![Crates.io](https://img.shields.io/crates/v/rust-i18n.svg)](https://crates.io/crates/rust-i18n)

Rust I18n is use Rust codegen for load YAML file storage translations on compile time, and give you a t! macro for simply get translation texts.

> Inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n).

### Usage

Load macro in your `lib.rs`

```rs
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
```

You must put I18n YAML files in `locales/` folder.

```bash
locales/
├── en.yml
├── zh-CN.yml
```

For example of `en.yml`:

```yml
en:
  hello: Hello world
  messages:
    hello: Hello, %{name}
```

Now you can use `t!` macro in anywhere.

```ignore
t!("hello");
// => "Hello world"

t!("hello", locale = "zh-CN);
// => "你好世界"

t!("messages.hello", name = "world");
// => "Hello, world"

t!("messages.hello", locale = "zh-CN", name = "Jason");
// => "你好, Jason"
```

You can use `rust_i18n::set_locale` to change the current locale in runtime.

```rs
rust_i18n::set_locale("zh-CN");
rust_i18n::locale();
// => "zh-CN"
```
*/
// include!(concat!(env!("OUT_DIR"), "/i18n.rs"));
use std::sync::Mutex;

pub use rust_i18n_support::i18n;

lazy_static::lazy_static! {
    static ref CURRENT_LOCALE: Mutex<String> = Mutex::new(String::from("en"));
}

pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = locale.to_string();
}

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
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        crate::translate(rust_i18n::locale().as_str(), $key)
    };

    // t!("foo", locale="en")
    ($key:expr, locale=$locale:tt) => {
        crate::translate($locale, $key)
    };

    // t!("foo", locale="en")
    ($key:expr, locale=$locale:tt, $($var_name:tt = $var_val:tt),+) => {
        {
            let mut message = crate::translate($locale, $key);
            $(
                message = message.replace(concat!("%{", stringify!($var_name), "}"), $var_val);
            )+
            message
        }
    };

    // t!("foo {} {}", "bar", "baz")
    ($key:expr, $($var_name:tt = $var_val:tt),+) => {
        {
            let mut message = crate::translate(rust_i18n::locale().as_str(), $key);
            $(
                message = message.replace(concat!("%{", stringify!($var_name), "}"), $var_val);
            )+
            message
        }
    };
}

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
