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

    let data = load_locales(&locales_path.display().to_string(), |_| false);
    let code = generate_code(data.translations, data.locales);

    if is_debug() {
        println!("{}", code.to_string());
    }

    code.into()
}

fn generate_code(
    translations: HashMap<String, String>,
    locales: Vec<String>,
) -> proc_macro2::TokenStream {
    let mut all_translations = Vec::<proc_macro2::TokenStream>::new();
    let mut all_locales = Vec::<proc_macro2::TokenStream>::new();
    // For keep locales unique
    let mut locale_names = HashMap::<String, String>::new();

    translations.iter().for_each(|(k, v)| {
        let k = k.to_string();
        let v = v.to_string();

        all_translations.push(quote! {
            #k => #v,
        });
    });

    locales.iter().for_each(|l| {
        if locale_names.contains_key(l) {
            return;
        }

        locale_names.insert(l.to_string(), l.to_string());
        all_locales.push(quote! {
            #l,
        });
    });

    // result
    quote! {
        static ALL_TRANSLATIONS: once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(|| rust_i18n::map! [
            #(#all_translations)*
            "" => ""
        ]);

        static AVAILABLE_LOCALES: &[&'static str] = &[
            #(#all_locales)*
        ];

        /// Get I18n text by locale and key
        pub fn translate(locale: &str, key: &str) -> String {
            let key = format!("{}.{}", locale, key);
            match ALL_TRANSLATIONS.get(key.as_str()) {
                Some(value) => value.to_string(),
                None => key.to_string(),
            }
        }

        /// Return all available locales, for example: `&["en", "zh-CN"]`
        pub fn available_locales() -> &'static [&'static str] {
            AVAILABLE_LOCALES
        }
    }
}
