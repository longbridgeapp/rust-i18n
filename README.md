# Rust I18n

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

```
locales/
├── en.yml
├── zh-CN.yml
```

For example of `en.yml`:

```yml
en:
  greeting: Hello world
  messages:
    hello: Hello, {}
```

Now you can use `t!` macro in anywhere.

```rs
t!("greeting");
// => "Hello world"

t!("messages.hello", "world");
// => "Hello, world"
```

You can use `rust_i18n::set_locale` or call `rust_i18n::init` agian to change the current locale in runtime.

```rs
rust_i18n::set_locale("zh-CN");
rust_i18n::locale();
// => "zh-CN"
```

### License

MIT
