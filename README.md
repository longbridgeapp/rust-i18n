# Rust I18n

[![CI](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml/badge.svg)](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml) [![Docs](https://docs.rs/rust-i18n/badge.svg)](https://docs.rs/rust-i18n/) [![Crates.io](https://img.shields.io/crates/v/rust-i18n.svg)](https://crates.io/crates/rust-i18n)

Rust I18n is use Rust codegen for load YAML file storage translations on compile time, and give you a t! macro for simply get translation texts.

> Inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n) and [Rails I18n](https://guides.rubyonrails.org/i18n.html).

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
  hello: Hello world
  messages:
    hello: Hello, %{name}
```

Now you can use `t!` macro in anywhere.

```rs
t!("hello");
// => "Hello world"

t!("hello", locale = "zh-CN");
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

## Debug codegen

Use `RUST_I18N_DEBUG` environment variable to run cargo build, Rust I18n will just print the codegen result.

```bash
$ RUST_I18N_DEBUG=1 cargo build
```

### License

MIT
