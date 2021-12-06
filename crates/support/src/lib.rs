use glob::glob;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Locale = String;
type Value = serde_json::Value;
type Translations = HashMap<Locale, Value>;

fn is_debug() -> bool {
    std::env::var("RUST_I18N_DEBUG").is_ok()
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

#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let option = match syn::parse::<Option>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let translations = load_locales(&option.locales_path);
    let code = generate_code(translations);

    if is_debug() {
        println!("{}", code.to_string());
        panic!("Debug mode, show codegen.");
    }

    code.into()
}

fn load_locales(dest: &str) -> Translations {
    let mut translations: Translations = HashMap::new();

    let current_dir = std::env::current_dir().unwrap();
    let locale_path = current_dir.join(dest);

    let locale_path = std::fs::canonicalize(locale_path.clone())
        .unwrap_or_else(|_| panic!("Invalid locale path: {}", &locale_path.display()))
        .display()
        .to_string();

    if is_debug() {
        println!("cargo:i18n-locale-path={}", &locale_path);
    }

    for entry in glob(&format!("{}/**/*.yml", locale_path)).expect("Failed to read glob pattern") {
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
