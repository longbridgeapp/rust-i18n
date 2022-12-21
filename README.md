# Rust I18n

[![CI](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml/badge.svg)](https://github.com/longbridgeapp/rust-i18n/actions/workflows/ci.yml) [![Docs](https://docs.rs/rust-i18n/badge.svg)](https://docs.rs/rust-i18n/) [![Crates.io](https://img.shields.io/crates/v/rust-i18n.svg)](https://crates.io/crates/rust-i18n)

Rust I18n is a crate for loading localized text from a set of YAML mapping files. The mappings are converted into data readable by Rust programs at compile time, and then localized text can be loaded by simply calling the provided `t!` macro.

The API of this crate is inspired by [ruby-i18n](https://github.com/ruby-i18n/i18n) and [Rails I18n](https://guides.rubyonrails.org/i18n.html).

## Features

- Codegen on compile time for includes translations into binary.
- Global `t!` macro for loading localized text in everywhere.
- Use YAML for mapping localized text, and support mutiple YAML files merging.
- `cargo i18n` Command line tool for checking and extract untranslated texts into YAML files.

## Installation

Rust I18n also provided a `cargo i18n` command line tool help you process translations.

```bash
$ cargo install rust-i18n
```

## Usage

Add crate dependencies in your Cargo.toml and setup I18n config:

```toml
[dependencies]
once_cell = "1.10.0"
rust-i18n = "1"

[package.metadata.i18n]
# The available locales for your application, default: ["en"].
# available-locales = ["en", "zh-CN"]

# The default locale, default: "en".
# default-locale = "en"

# Path for your translations YAML file, default: "locales".
# load-path = "locales"
```

Load macro and init translations in `lib.rs`

```rs
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");
```

Or you can import by use directly:

```rs
// You must import in each files when you wants use `t!` macro.
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    println!("{}", t!("hello"));
}
```

Make sure all YAML files (containing the localized mappings) are located in the `locales/` folder of the project root directory:

```
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
└───────locales
└───────src
└───────Cargo.toml
```

In the YAML files, specify the localization keys and their corresponding values, for example, in `en.yml`:

```yml
hello: Hello world # A simple key -> value mapping
messages:
  hello: Hello, %{name} # A nested key.sub_key -> value mapping, in this case "messages.hello" maps to "Hello, %{name}"
```

And example of the `zh-CN.yml`:

```yml
hello: 你好世界
messages:
  hello: 你好，%{name}
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

t!("messages.hello", locale = "zh-CN", name = "Jason");
// => "你好，Jason"
```

### Setting and Getting the Global Locale

You can use `rust_i18n::set_locale` to set the global locale at runtime, so that you don't have to specify the locale on each `t!` invocation.

```rs
rust_i18n::set_locale("zh-CN");

let locale = rust_i18n::locale();
assert_eq!(locale, "zh-CN");
```

### Extract the untranslated texts

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

## License

MIT
