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
use glob::glob;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Mutex, MutexGuard, Once};

type Locale = String;
type Value = serde_json::Value;
type Translations = HashMap<Locale, Value>;

/// Merge JSON Values, merge b into a
fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

#[derive(Debug)]
struct Option {
    locales_path: String,
}

impl syn::parse::Parse for Option {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let locales_path = input.parse::<syn::LitStr>()?.value();

        Ok(Self { locales_path })
    }
}

fn is_debug() -> bool {
    std::env::var("RUST_I18N_DEBUG").is_ok()
}

static INITIALIZED: Once = Once::new();

lazy_static::lazy_static! {
    static ref TRANSLATIONS: Mutex<Translations> = Mutex::new(Translations::new());
}

#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let option = match syn::parse::<Option>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let new_translations = load_locales(&option.locales_path);

    let mut translations = TRANSLATIONS.lock().unwrap();
    new_translations.into_iter().for_each(|(k, new_value)| {
        translations
            .entry(k)
            .and_modify(|old_value| merge_value(old_value, &new_value))
            .or_insert(new_value);
    });

    // if is_debug() {
    //     println!("{:#?}", translations);
    // }

    let mut code = generate_code(translations);

    INITIALIZED.call_once(|| {
        let base_code = generate_base();

        code = quote! {
            #base_code
            #code
        };
    });

    if is_debug() {
        println!("Code generated:\n{}", code.to_string());
    }

    code.into()
}

fn load_locales(dest: &str) -> Translations {
    let mut translations: Translations = HashMap::new();

    let locale_path = format!("{}/**/*.yml", dest);

    if is_debug() {
        println!("cargo:i18n-locale-path={}", &locale_path);
    }

    for entry in glob(&locale_path).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        if is_debug() {
            println!("cargo:rerun-if-changed={}", entry.display());
        }

        let file = File::open(entry).expect("Failed to open the YAML file");
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();

        reader
            .read_to_string(&mut content)
            .expect("Read YAML file failed.");

        let trs: Translations =
            serde_yaml::from_str(&content).expect("Invalid YAML format, parse error");

        translations.extend(trs)
    }

    translations
}

fn extract_vars(prefix: &str, trs: &Value) -> HashMap<String, String> {
    let mut v = HashMap::<String, String>::new();
    let prefix = prefix.to_string();

    match &trs {
        serde_json::Value::String(s) => {
            v.insert(prefix, s.to_string());
        }
        serde_json::Value::Object(o) => {
            for (k, vv) in o {
                let key = format!("{}.{}", prefix, k);
                v.extend(extract_vars(key.as_str(), vv));
            }
        }
        serde_json::Value::Null => {
            v.insert(prefix, "".into());
        }
        serde_json::Value::Bool(s) => {
            v.insert(prefix, format!("{}", s));
        }
        serde_json::Value::Number(s) => {
            v.insert(prefix, format!("{}", s));
        }
        serde_json::Value::Array(_) => {
            v.insert(prefix, "".into());
        }
    }

    v
}

fn generate_base() -> proc_macro2::TokenStream {
    let result = quote! {
        use std::sync::Mutex;
        use std::collections::HashMap;


        lazy_static::lazy_static! {
            static ref CURRENT_LOCALE: Mutex<String> = Mutex::new(String::from("en"));
        }

        macro_rules! map {
            {$($key:expr => $value:expr),+} => {{
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }};
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
                $crate::translate($crate::locale().as_str(), $key)
            };

            // t!("foo", locale="en")
            ($key:expr, locale=$locale:tt) => {
                $crate::translate($locale, $key)
            };

            // t!("foo", locale="en")
            ($key:expr, locale=$locale:tt, $($var_name:tt = $var_val:tt),+) => {
                {
                    let mut message = $crate::translate($locale, $key);
                    $(
                        message = message.replace(concat!("%{", stringify!($var_name), "}"), $var_val);
                    )+
                    message
                }
            };

            // t!("foo {} {}", "bar", "baz")
            ($key:expr, $($var_name:tt = $var_val:tt),+) => {
                {
                    let mut message = $crate::translate($crate::locale().as_str(), $key);
                    $(
                        message = message.replace(concat!("%{", stringify!($var_name), "}"), $var_val);
                    )+
                    message
                }
            };
        }

        pub fn translate(locale: &str, key: &str) -> String {
            let key = format!("{}.{}", locale, key);
            match LOCALES.get(key.as_str()) {
                Some(value) => value.to_string(),
                None => {
                    key.to_string()
                }
            }
        }
    };

    result
}

fn generate_code(translations: MutexGuard<HashMap<String, Value>>) -> proc_macro2::TokenStream {
    let mut locales = Vec::<TokenStream>::new();

    let mut locale_vars = HashMap::<String, String>::new();
    translations.iter().for_each(|(locale, trs)| {
        let new_vars = extract_vars(locale.as_str(), &trs);
        locale_vars.extend(new_vars);
    });

    locale_vars.iter().for_each(|(k, v)| {
        let k = k.to_string();
        let v = v.to_string();

        locales.push(quote! {
            #k => #v,
        });
    });

    if is_debug() {
        println!("cargo:i18n-locales={:#?}", locales);
    }

    let result = quote! {
        lazy_static::lazy_static! {
            static ref LOCALES: HashMap<&'static str, &'static str> = map! [
                #(#locales)*
                "" => ""
            ];
        }
    };

    result
}
