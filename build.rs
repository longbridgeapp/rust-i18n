use glob::glob;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Locale = String;
type Value = serde_json::Value;
type Translations = HashMap<Locale, Value>;

/*
    Inspired from https://github.com/terry90/internationalization-rs/blob/master/build.rs
*/

fn load_locales() -> Translations {
    let mut translations: Translations = HashMap::new();

    let build_directory = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let locales = format!("{}/**/locales/**/*.yml", build_directory);
    println!("Reading {}", &locales);

    for entry in glob(&locales).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.display());

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

#[allow(unused)]
fn debug<T: ?Sized + serde::Serialize>(val: &T) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("target/rust-i18n-debug.log")
        .unwrap();

    writeln!(file, "{}", serde_json::to_string_pretty(val).unwrap()).unwrap();
}

fn generate_code(translations: Translations) -> proc_macro2::TokenStream {
    let mut locales = Vec::<TokenStream>::new();

    let mut locale_vars = HashMap::<String, String>::new();
    for (locale, trs) in translations {
        let new_vars = extract_vars(locale.as_str(), &trs);
        locale_vars.extend(new_vars);
    }

    locale_vars.iter().for_each(|(k, v)| {
        let k = k.to_string();
        let v = v.to_string();

        locales.push(quote! {
            #k => #v,
        });
    });

    let result = quote! {
        use std::sync::Mutex;
        use std::collections::HashMap;

        macro_rules! map {
            {$($key:expr => $value:expr),+} => {{
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }};
        }

        lazy_static::lazy_static! {
            static ref LOCALES: HashMap<&'static str, &'static str> = map! [
                #(#locales)*
                "" => ""
            ];

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

fn write_code(code: TokenStream) {
    let dest = std::env::var("OUT_DIR").unwrap();
    let mut output = File::create(&std::path::Path::new(&dest).join("i18n.rs")).unwrap();
    output
        .write(code.to_string().as_bytes())
        .expect("Write generated i18n.rs error");
}

fn main() {
    let translations = load_locales();
    let code = generate_code(translations);
    if std::env::var("RUST_I18N_DEBUG").is_ok() {
        debug(&code.to_string());
    }

    write_code(code);
}

mod tests {
    #[test]
    fn test_extract_vars() {
        let translations = read_locales();
        let code = generate_code(translations);

        assert_eq!("", code);
    }
}
