# Rust I18n

[![CI](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml/badge.svg)](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml) [![Docs](https://docs.rs/rust-i18n/badge.svg)](https://docs.rs/rust-i18n/) [![Crates.io](https://img.shields.io/crates/v/rust-i18n.svg)](https://crates.io/crates/rust-i18n)

> 🎯 Let's make I18n things to easy!

Rust I18n is a crate for loading localized text from a set of (YAML, JSON or TOML) mapping files. The mappings are converted into data readable by Rust programs at compile time, and then localized text can be loaded by simply calling the provided `t!` macro.

Unlike other I18n libraries, Rust I18n's goal is to provide a simple and easy-to-use API.

The API of this crate is inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n) and [Rails I18n](https://guides.rubyonrails.org/i18n.html).

## Features

- Codegen on compile time for includes translations into binary.
- Global `t!` macro for loading localized text in everywhere.
- Use YAML (default), JSON or TOML format for mapping localized text, and support mutiple files merging.
- `cargo i18n` Command line tool for checking and extract untranslated texts into YAML files.

## Usage

Add crate dependencies in your Cargo.toml and setup I18n config:

```toml
[dependencies]
rust-i18n = "1"
```

Load macro and init translations in `lib.rs`

```rs
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");

// Or just use `i18n!`, default locales path is: "locales" in current crate.
i18n!();

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
i18n!("locales", fallback = "en");
```

Or you can import by use directly:

```rs
// You must import in each files when you wants use `t!` macro.
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    println!("{}", t!("hello"));

    // Use `available_locales!` method to get all available locales.
    println!("{:?}", rust_i18n::available_locales!());
}
```

Make sure all localized files (containing the localized mappings) are located in the `locales/` folder of the project root directory:

> 💡Since: v1.3.0, the localized files supports use multiple formats, including `*.{yml,yaml,json,toml}`, and will merge all them.

```bash
.
├── Cargo.lock
├── Cargo.toml
├── locales
│   ├── en.yml
│   ├── zh-CN.yml
│   └── zh-HK.yml
└── src
│   └── main.rs
└── sub_app
│   └── locales
│   │   └── en.yml
│   │   └── zh-CN.yml
│   │   └── zh-HK.yml
│   └── src
│   │   └── main.rs
│   └── Cargo.toml
```

In the localized files, specify the localization keys and their corresponding values, for example, in `en.yml`:

```yml
hello: Hello world # A simple key -> value mapping
messages:
  hello: Hello, %{name} # A nested key.sub_key -> value mapping, in this case "messages.hello" maps to "Hello, %{name}"
```

And example of the `zh-CN.yml`:

```yml
hello: 你好世界
messages:
  hello: 你好，%{name} (%{count})
```

If you wants use JSON format, just rename the file to `en.json` and the content is like this:

```json
{
  "hello": "Hello world",
  "messages": {
    "hello": "Hello, %{name}"
  }
}
```

Or use TOML format, just rename the file to `en.toml` and the content is like this:

```toml
hello = "Hello world"

[messages]
hello = "Hello, %{name}"
```

### Loading Localized Strings in Rust

Import the `t!` macro from this crate into your current scope:

```rs
use rust_i18n::t;
```

Then, simply use it wherever a localized string is needed:

```rs
t!("hello");
// => "Hello world"

t!("hello", locale = "zh-CN");
// => "你好世界"

t!("messages.hello", name = "world");
// => "Hello, world"

t!("messages.hello", "name" => "world");
// => "Hello, world"

t!("messages.hello", locale = "zh-CN", name = "Jason", count = 2);
// => "你好，Jason (2)"

t!("messages.hello", locale = "zh-CN", "name" => "Jason", "count" => 3 + 2);
// => "你好，Jason (5)"
```

### Setting and Getting the Global Locale

You can use `rust_i18n::set_locale` to set the global locale at runtime, so that you don't have to specify the locale on each `t!` invocation.

```rs
rust_i18n::set_locale("zh-CN");

let locale = rust_i18n::locale();
assert_eq!(locale, "zh-CN");
```

## Extractor

We provided a `cargo i18n` command line tool for help you extract the untranslated texts from the source code and then write into YAML file.

You can install it via `cargo install rust-i18n`, then you get `cargo i18n` command.

```bash
$ cargo install rust-i18n
```

### Extend Backend

Since v2.0 rust-i18n support extend backend for cusomize your translation implementation.

For example, you can use HTTP API for load translations from remote server:

```rs
use rust_i18n::Backend;

pub struct RemoteI18n {
    trs: HashMap<String, HashMap<String, String>>,
}

impl RemoteI18n {
    fn new() -> Self {
        // fetch translations from remote URL
        let response = reqwest::blocking::get("https://your-host.com/assets/locales.yml").unwrap();
        let trs = serde_yaml::from_str::<HashMap<String, HashMap<String, String>>>(&response.text().unwrap()).unwrap();

        return Self {
            trs
        };
    }
}

impl Backend for RemoteI18n {
    fn available_locales(&self) -> Vec<String> {
        return self.trs.keys().cloned().collect();
    }

    fn translate(&self, locale: &str, key: &str) -> Option<String> {
        // Write your own lookup logic here.
        // For example load from database
        return self.trs.get(locale)?.get(key).cloned();
    }
}
```

Now you can init rust_i18n by extend your own backend:

```rs
rust_i18n::i18n!("locales", backend = RemoteI18n::new());
```

This also will load local translates from ./locales path, but your own `RemoteI18n` will priority than it.

Now you call `t!` will lookup translates from your own backend first, if not found, will lookup from local files.

### Configuration for `cargo i18n` command

💡 NOTE: `package.metadata.i18n` config section in Cargo.toml is just work for `cargo i18n` command, if you don't use that, you don't need this config.

```toml
[package.metadata.i18n]
# The available locales for your application, default: ["en"].
# available-locales = ["en", "zh-CN"]

# The default locale, default: "en".
# default-locale = "en"

# Path for your translations YAML file, default: "locales".
# This config for let `cargo i18n` command line tool know where to find your translations.
# You must keep this path same as the one you pass to method `rust_i18n::i18n!`.
# load-path = "locales"
```

Rust I18n providered a `i18n` bin for help you extract the untranslated texts from the source code and then write into YAML file.

```bash
$ cargo install rust-i18n
# Now you have `cargo i18n` command
```

After that the untranslated texts will be extracted and saved into `locales/TODO.en.yml` file.

You also can special the locale by use `--locale` option:

```bash
$ cd your_project_root_directory
$ cargo i18n

Checking [en] and generating untranslated texts...
Found 1 new texts need to translate.
----------------------------------------
Writing to TODO.en.yml

Checking [fr] and generating untranslated texts...
Found 11 new texts need to translate.
----------------------------------------
Writing to TODO.fr.yml

Checking [zh-CN] and generating untranslated texts...
All thing done.

Checking [zh-HK] and generating untranslated texts...
Found 11 new texts need to translate.
----------------------------------------
Writing to TODO.zh-HK.yml
```

Run `cargo i18n -h` to see details.

```bash
$ cargo i18n -h
cargo-i18n 0.5.0
---------------------------------------
Rust I18n command for help you simply to extract all untranslated texts from soruce code.

It will iter all Rust files in and extract all untranslated texts that used `t!` macro.
And then generate a YAML file and merge for existing texts.

https://github.com/longbridgeapp/rust-i18n

USAGE:
    cargo i18n [OPTIONS] [--] [source]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    Path of your Rust crate root [default: ./]
```

## Debugging the Codegen Process

The `RUST_I18N_DEBUG` environment variable can be used to print out some debugging infos when code is being generated at compile time.

```bash
$ RUST_I18N_DEBUG=1 cargo build
```

## Example

A minimal example of using rust-i18n can be found [here](https://github.com/longbridgeapp/rust-i18n/tree/main/examples).

## I18n Ally

I18n Ally is a VS Code extension for helping you translate your Rust project.

You can add [i18n-ally-custom-framework.yml](https://github.com/longbridgeapp/rust-i18n/blob/main/.vscode/i18n-ally-custom-framework.yml) to your project `.vscode` directory, and then use I18n Ally can parse `t!` marco to show translate text in VS Code editor.

## Benchmark

Benchmark `t!` method, result on Apple M1:

```bash
t                       time:   [100.91 ns 101.06 ns 101.24 ns]
t_with_args             time:   [495.56 ns 497.88 ns 500.64 ns]
```

The result `101 ns (0.0001 ms)` means if there have 10K translate texts, it will cost 1ms.

## License

MIT
