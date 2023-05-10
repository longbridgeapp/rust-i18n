use quote::quote;
use rust_i18n_support::{is_debug, load_locales};
use std::collections::HashMap;
use syn::{parse_macro_input, Ident, LitStr, Token};

#[derive(Debug)]
struct Args {
    locales_path: String,
    fallback: Option<String>,
}

impl Args {
    fn consume_path(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let path = input.parse::<LitStr>()?;
        self.locales_path = path.value();

        Ok(())
    }

    fn consume_options(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let val = input.parse::<LitStr>()?.value();

        if ident == "fallback" {
            self.fallback = Some(val);
        }

        Ok(())
    }
}

impl syn::parse::Parse for Args {
    /// Parse macro arguments.
    ///
    /// ```ignore
    /// i18n!();
    /// i18n!("locales");
    /// i18n!("locales", fallback = "en");
    /// ```
    ///
    /// Ref: https://docs.rs/syn/latest/syn/parse/index.html
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();

        let mut result = Self {
            locales_path: String::from("locales"),
            fallback: None,
        };

        if lookahead.peek(LitStr) {
            result.consume_path(input)?;

            if input.parse::<Token![,]>().is_ok() {
                result.consume_options(input)?;
            }
        } else if lookahead.peek(Ident) {
            result.consume_options(input)?;
        }

        Ok(result)
    }
}

/// Init I18n translations.
///
/// This will load all translations by glob `**/*.yml` from the given path, default: `${CARGO_MANIFEST_DIR}/locales`.
///
/// Attribute `fallback` for set the fallback locale, if present `t` macro will use it as the fallback locale.
///
/// ```ignore
/// i18n!();
/// i18n!("locales");
/// i18n!("locales", fallback = "en");
/// ```
#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);

    // CARGO_MANIFEST_DIR is current build directory
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join(args.locales_path);

    let data = load_locales(&locales_path.display().to_string(), |_| false);
    let code = generate_code(data.translations, data.locales, args.fallback);

    if is_debug() {
        println!("{}", code.to_string());
    }

    code.into()
}

fn generate_code(
    translations: HashMap<String, String>,
    locales: Vec<String>,
    fallback: Option<String>,
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

    let fallback = if let Some(fallback) = fallback {
        quote! {
            Some(#fallback)
        }
    } else {
        quote! {
            None
        }
    };

    // result
    quote! {
        static _RUST_I18N_ALL_TRANSLATIONS: once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(|| rust_i18n::map! [
            #(#all_translations)*
            "" => ""
        ]);

        static _RUST_I18N_AVAILABLE_LOCALES: &[&'static str] = &[
            #(#all_locales)*
        ];

        static _RUST_I18N_FALLBACK_LOCALE: Option<&'static str> = #fallback;

        pub fn translate(locale: &str, key: &str) -> String {
            _rust_i18n_translate(locale, key)
        }

        /// Get I18n text by locale and key
        pub fn _rust_i18n_translate(locale: &str, key: &str) -> String {
            let target_key = format!("{}.{}", locale, key);
            if let Some(value) = _RUST_I18N_ALL_TRANSLATIONS.get(target_key.as_str()) {
                return value.to_string();
            }


            if let Some(fallback) = _RUST_I18N_FALLBACK_LOCALE {
                let fallback_key = format!("{}.{}", fallback, key);
                if let Some(value) = _RUST_I18N_ALL_TRANSLATIONS.get(fallback_key.as_str()) {
                    return value.to_string();
                }
            }

            return target_key
        }

        /// Return all available locales, for example: `&["en", "zh-CN"]`
        pub fn available_locales() -> &'static [&'static str] {
            _RUST_I18N_AVAILABLE_LOCALES
        }
    }
}
