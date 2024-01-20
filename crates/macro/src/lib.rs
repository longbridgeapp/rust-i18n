use quote::quote;
use rust_i18n_support::{is_debug, load_locales};
use std::collections::HashMap;
use syn::{parse_macro_input, Expr, Ident, LitStr, Token};

mod tr;

struct Args {
    locales_path: String,
    fallback: Option<Vec<String>>,
    extend: Option<Expr>,
}

impl Args {
    fn consume_path(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let path = input.parse::<LitStr>()?;
        self.locales_path = path.value();

        Ok(())
    }

    fn consume_fallback(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        if let Ok(val) = input.parse::<LitStr>() {
            self.fallback = Some(vec![val.value()]);
            return Ok(());
        }
        let val = input.parse::<syn::ExprArray>()?;
        let fallback = val
            .elems
            .into_iter()
            .map(|expr| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = expr
                {
                    Ok(lit_str.value())
                } else {
                    Err(input.error(
                        "`fallback` must be a string literal or an array of string literals",
                    ))
                }
            })
            .collect::<syn::parse::Result<Vec<String>>>()?;
        self.fallback = Some(fallback);
        Ok(())
    }

    fn consume_options(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let ident = input.parse::<Ident>()?.to_string();
        input.parse::<Token![=]>()?;

        match ident.as_str() {
            "fallback" => {
                self.consume_fallback(input)?;
            }
            "backend" => {
                let val = input.parse::<Expr>()?;
                self.extend = Some(val);
            }
            _ => {}
        }

        // Continue to consume reset of options
        if input.parse::<Token![,]>().is_ok() {
            self.consume_options(input)?;
        }

        Ok(())
    }
}

impl syn::parse::Parse for Args {
    /// Parse macro arguments.
    ///
    /// ```no_run
    /// # use rust_i18n::i18n;
    /// # fn v1() {
    /// i18n!();
    /// # }
    /// # fn v2() {
    /// i18n!("locales");
    /// # }
    /// # fn v3() {
    /// i18n!("locales", fallback = "en");
    /// # }
    /// # fn v4() {
    /// i18n!("locales", fallback = ["en", "es"]);
    /// # }
    /// ```
    ///
    /// Ref: https://docs.rs/syn/latest/syn/parse/index.html
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();

        let mut result = Self {
            locales_path: String::from("locales"),
            fallback: None,
            extend: None,
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
/// ```no_run
/// # use rust_i18n::i18n;
/// # fn v1() {
/// i18n!();
/// # }
/// # fn v2() {
/// i18n!("locales");
/// # }
/// # fn v3() {
/// i18n!("locales", fallback = "en");
/// # }
/// # fn v4() {
/// i18n!("locales", fallback = ["en", "es"]);
/// # }
/// ```
#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);

    // CARGO_MANIFEST_DIR is current build directory
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join(&args.locales_path);

    let data = load_locales(&locales_path.display().to_string(), |_| false);
    let code = generate_code(data, args);

    if is_debug() {
        println!(
            "\n\n-------------- code --------------\n{}\n----------------------------------\n\n",
            code
        );
    }

    code.into()
}

fn generate_code(
    translations: HashMap<String, HashMap<String, String>>,
    args: Args,
) -> proc_macro2::TokenStream {
    let mut all_translations = Vec::<proc_macro2::TokenStream>::new();

    translations.iter().for_each(|(locale, trs)| {
        let mut sub_trs = Vec::<proc_macro2::TokenStream>::new();

        trs.iter().for_each(|(k, v)| {
            let k = k.to_string();
            let v = v.to_string();
            sub_trs.push(quote! {
                (#k, #v)
            });
        });

        all_translations.push(quote! {
            let trs = [#(#sub_trs),*];
            backend.add_translations(#locale, &trs.into_iter().collect());
        });
    });

    let fallback = if let Some(fallback) = args.fallback {
        quote! {
            Some(&[#(#fallback),*])
        }
    } else {
        quote! {
            None
        }
    };

    let extend_code = if let Some(extend) = args.extend {
        quote! {
            let backend = backend.extend(#extend);
        }
    } else {
        quote! {}
    };

    // result
    quote! {
        use rust_i18n::{BackendExt, CowStr, TrKey};
        use std::borrow::Cow;

        /// I18n backend instance
        ///
        /// [PUBLIC] This is a public API, and as an example in examples/
        #[allow(missing_docs)]
        static _RUST_I18N_BACKEND: rust_i18n::once_cell::sync::Lazy<Box<dyn rust_i18n::Backend>> = rust_i18n::once_cell::sync::Lazy::new(|| {
            let mut backend = rust_i18n::SimpleBackend::new();
            #(#all_translations)*
            #extend_code

            Box::new(backend)
        });

        static _RUST_I18N_FALLBACK_LOCALE: Option<&[&'static str]> = #fallback;

        /// Lookup fallback locales
        ///
        /// For example: `"zh-Hant-CN-x-private1-private2"` -> `"zh-Hant-CN-x-private1"` -> `"zh-Hant-CN"` -> `"zh-Hant"` -> `"zh"`.
        ///
        /// https://datatracker.ietf.org/doc/html/rfc4647#section-3.4
        #[inline]
        #[allow(missing_docs)]
        pub fn _rust_i18n_lookup_fallback(locale: &str) -> Option<&str> {
            locale.rfind('-').map(|n| locale[..n].trim_end_matches("-x"))
        }

        /// Get I18n text by locale and key
        #[inline]
        #[allow(missing_docs)]
        pub fn _rust_i18n_translate<'r>(locale: &str, key: &'r str) -> Cow<'r, str> {
            _rust_i18n_try_translate(locale, key).unwrap_or_else(|| {
                if locale.is_empty() {
                    key.into()
                } else {
                    format!("{}.{}", locale, key).into()
                }
            })
        }

        /// Try to get I18n text by locale and key
        #[inline]
        #[allow(missing_docs)]
        pub fn _rust_i18n_try_translate<'r>(locale: &str, key: impl AsRef<str>) -> Option<Cow<'r, str>> {
            _RUST_I18N_BACKEND.translate(locale, key.as_ref())
                .map(Cow::from)
                .or_else(|| {
                    let mut current_locale = locale;
                    while let Some(fallback_locale) = _rust_i18n_lookup_fallback(current_locale) {
                        if let Some(value) = _RUST_I18N_BACKEND.translate(fallback_locale, key.as_ref()) {
                            return Some(Cow::from(value));
                        }
                        current_locale = fallback_locale;
                    }

                    _RUST_I18N_FALLBACK_LOCALE.and_then(|fallback| {
                        fallback.iter().find_map(|locale| _RUST_I18N_BACKEND.translate(locale, key.as_ref()).map(Cow::from))
                    })
                })
        }

        #[allow(missing_docs)]
        pub fn _rust_i18n_available_locales() -> Vec<&'static str> {
            let mut locales = _RUST_I18N_BACKEND.available_locales();
            locales.sort();
            locales
        }
    }
}

/// A procedural macro that generates a string representation of the input.
///
/// This macro accepts either a string literal or an identifier as input.
/// If the input is a string literal, it returns the value of the string literal.
/// If the input is an identifier, it returns the string representation of the identifier.
///
/// # Arguments
///
/// * `input` - The input token stream. It should be either a string literal or an identifier.
///
/// # Returns
///
/// Returns a token stream that contains a string representation of the input. If the input cannot be parsed as a string literal or an identifier,
/// it returns a compile error.
///
/// # Example
///
/// ```no_run
/// # use rust_i18n::vakey;
/// # fn v1() {
/// let key = vakey!(name);
/// # }
/// # fn v2() {
/// let key = vakey!("name");
/// # }
/// ```
#[proc_macro]
pub fn vakey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = syn::parse::<syn::LitStr>(input.clone())
        .map(|str| str.value())
        .or(syn::parse::<syn::Ident>(input.clone()).map(|ident| format!("{}", ident)));

    match output {
        Ok(value) => quote! { #value }.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Get I18n text with literals supports.
///
/// This macro first checks if a translation exists for the input string.
/// If it does, it returns the translated string.
/// If it does not, it returns the input string literal.
///
/// # Variants
///
/// This macro has several variants that allow for different use cases:
///
/// * `tr!("foo")`:
///   Translates the string "foo" using the current locale.
///
/// * `tr!("foo", locale = "en")`:
///   Translates the string "foo" using the specified locale "en".
///
/// * `tr!("foo", locale = "en", a = 1, b = "Foo")`:
///   Translates the string "foo" using the specified locale "en" and replaces the patterns "{a}" and "{b}" in the string with "1" and "Foo" respectively.
///
/// * `tr!("foo %{a} %{b}", a = "bar", b = "baz")`:
///   Translates the string "foo %{a} %{b}" using the current locale and replaces the patterns "{a}" and "{b}" in the string with "bar" and "baz" respectively.
///
/// * `tr!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")`:
///   Translates the string "foo %{a} %{b}" using the specified locale "en" and replaces the patterns "{a}" and "{b}" in the string with "bar" and "baz" respectively.
///
/// * `tr!("foo %{a} %{b}", "a" => "bar", "b" => "baz")`:
///   Translates the string "foo %{a} %{b}" using the current locale and replaces the patterns "{a}" and "{b}" in the string with "bar" and "baz" respectively.
///
/// # Examples
///
/// ```no_run
/// #[macro_use] extern crate rust_i18n;
/// # use rust_i18n::{tr, CowStr};
/// # fn _rust_i18n_try_translate<'r>(locale: &str, key: &'r str) -> Option<std::borrow::Cow<'r, str>> { todo!() }
/// # fn main() {
/// // Simple get text with current locale
/// tr!("Hello world");
/// // => "Hello world" (Key `tr_3RnEdpgZvZ2WscJuSlQJkJ` for "Hello world")
///
/// // Get a special locale's text
/// tr!("Hello world", locale = "de");
/// // => "Hallo Welt!" (Key `tr_3RnEdpgZvZ2WscJuSlQJkJ` for "Hello world")
///
/// // With variables
/// tr!("Hello, %{name}", name = "world");
/// // => "Hello, world" (Key `tr_4Cct6Q289b12SkvF47dXIx` for "Hello, %{name}")
/// tr!("Hello, %{name} and %{other}", name = "Foo", other ="Bar");
/// // => "Hello, Foo and Bar" (Key `tr_3eULVGYoyiBuaM27F93Mo7` for "Hello, %{name} and %{other}")
///
/// // With locale and variables
/// tr!("Hallo, %{name}", locale = "de", name => "Jason"); // Arrow style
/// tr!("Hallo, %{name}", locale = "de", name = "Jason"); // Asignment style
/// tr!("Hallo, %{name}", locale = "de", name : "Jason"); // Colon style
/// // => "Hallo, Jason" (Key `tr_4Cct6Q289b12SkvF47dXIx` for "Hallo, %{name}")
/// # }
/// ```
#[proc_macro]
pub fn tr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as tr::Tr).into()
}
