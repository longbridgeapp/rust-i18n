use quote::quote;
use rust_i18n_support::{is_debug, load_locales};
use std::collections::HashMap;

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

    let translations = load_locales(&locales_path.display().to_string(), |_| false);
    let code = generate_code(translations);

    if is_debug() {
        println!("{}", code.to_string());
        // panic!("Debug mode, show codegen.");
    }

    code.into()
}

fn generate_code(translations: HashMap<String, String>) -> proc_macro2::TokenStream {
    let mut locales = Vec::<proc_macro2::TokenStream>::new();

    translations.iter().for_each(|(k, v)| {
        let k = k.to_string();
        let v = v.to_string();

        locales.push(quote! {
            #k => #v,
        });
    });

    // result
    quote! {
        lazy_static::lazy_static! {
            static ref LOCALES: std::collections::HashMap<&'static str, &'static str> = rust_i18n::map! [
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
