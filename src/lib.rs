/*!
[![CI](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml/badge.svg)](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml) [![Docs](https://docs.rs/rust-i18n/badge.svg)](https://docs.rs/rust-i18n/) [![Crates.io](https://img.shields.io/crates/v/rust-i18n.svg)](https://crates.io/crates/rust-i18n)

Loon is a localization/internationalization library, inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n).

It use [rust-embed](https://crates.io/crates/rust-embed) for embed the localization assets into your binary.

### Usage

Load locales assets by RustEmbed and init Loon in your `lib.rs`

```ignore
use rust_embed::RustEmbed;

// Load Loon macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate loon_embed;

// Use RustEmbed to locale assets
#[derive(RustEmbed)]
#[folder = "locales/"]
#[include = "*.yml"]
struct Asset;

fn main() {
    loon_embed::init::<Asset>("en");
}
```

You must put I18n YAML files in `locales/` folder.

```bash
locales/
├── en.yml
├── zh-CN.yml
```

For example of `en.yml`:

```yml
greeting: Hello world
messages:
  hello: Hello, {}
```

Now you can use `t!` macro in anywhere.

```ignore
t!("greeting");
// => "Hello world"

t!("messages.hello", "world");
// => "Hello, world"
```

You can use `loon_embed::set_locale` or call `loon_embed::init` agian to change the current locale in runtime.

```rs
loon_embed::set_locale("zh-CN");
loon_embed::locale();
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
