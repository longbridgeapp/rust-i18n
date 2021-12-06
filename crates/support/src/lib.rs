use glob::glob;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Locale = String;
type Value = serde_json::Value;
type Translations = HashMap<Locale, Value>;

fn is_debug() -> bool {
    std::env::var("RUST_I18N_DEBUG").unwrap_or_else(|_| "0".to_string()) == "1"
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

/// Init I18n translations.
///
/// This will load all translations by glob `**/*.yml` from the given path.
///
/// ```ignore
/// i18n!("locales");
/// ```
#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let option = match syn::parse::<Option>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    // CARGO_MANIFEST_DIR is current build directory
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join(option.locales_path);

    let translations = load_locales(&locales_path.display().to_string());
    let code = generate_code(translations);

    if is_debug() {
        println!("{}", code.to_string());
        // panic!("Debug mode, show codegen.");
    }

    code.into()
}

fn load_locales(locales_path: &str) -> Translations {
    let mut translations: Translations = HashMap::new();

    let path_pattern = format!("{}/**/*.yml", locales_path);

    if is_debug() {
        println!("cargo:i18n-locale-path={}", &path_pattern);
    }

    for entry in glob(&path_pattern).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        if is_debug() {
            println!("cargo:i18n-load={}", &entry.display());
        }
        println!("cargo:rerun-if-changed={}", entry.display());

        let file = File::open(entry).expect("Failed to open the YAML file");
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();

        reader
            .read_to_string(&mut content)
            .expect("Read YAML file failed.");

        let trs: Translations =
            serde_yaml::from_str(&content).expect("Invalid YAML format, parse error");

        trs.into_iter().for_each(|(k, new_value)| {
            translations
                .entry(k)
                .and_modify(|old_value| merge_value(old_value, &new_value))
                .or_insert(new_value);
        });
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

fn generate_code(translations: Translations) -> proc_macro2::TokenStream {
    let mut locales = Vec::<proc_macro2::TokenStream>::new();

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

    // result
    quote! {
        lazy_static::lazy_static! {
            static ref LOCALES: std::collections::HashMap<&'static str, &'static str> = map! [
                #(#locales)*
                "" => ""
            ];
        }


        pub fn translate(locale: &str, key: &str) -> String {
            let key = format!("{}.{}", locale, key);
            match LOCALES.get(key.as_str()) {
                Some(value) => value.to_string(),
                None => key.to_string(),
            }
        }
    }
}
