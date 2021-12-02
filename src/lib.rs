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
include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_t() {
        assert_eq!(t!("hello"), "Hello, World!");
        assert_eq!(t!("hello"), "Hello, World!");

        // Vars
        assert_eq!(
            t!("a.very.nested.message"),
            "Hello, %{name}. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason"),
            "Hello, Jason. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason", msg = "Bla bla"),
            "Hello, Jason. Your message is: Bla bla"
        );

        crate::set_locale("de");
        assert_eq!(t!("messages.hello", name = "world"), "Hallo, world!");

        crate::set_locale("en");
        assert_eq!(t!("messages.hello", name = "world"), "Hello, world!");
    }

    #[test]
    fn it_t_with_locale_and_args() {
        assert_eq!(t!("hello", locale = "de"), "Hallo Welt!");
        assert_eq!(t!("hello", locale = "en"), "Hello, World!");

        assert_eq!(t!("messages.hello", name = "Jason"), "Hello, Jason!");
        assert_eq!(
            t!("messages.hello", locale = "en", name = "Jason"),
            "Hello, Jason!"
        );
        assert_eq!(
            t!("messages.hello", locale = "de", name = "Jason"),
            "Hallo, Jason!"
        );
    }
}
