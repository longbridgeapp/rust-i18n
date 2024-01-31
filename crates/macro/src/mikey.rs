use quote::quote;
use rust_i18n_support::minify_key;
use syn::Token;

/// A type representing the `mikey!` proc macro.
#[derive(Clone, Debug, Default)]
pub struct MiKey {
    msg: String,
    len: usize,
    prefix: String,
    threshold: usize,
}

impl MiKey {
    fn into_token_stream(self) -> proc_macro2::TokenStream {
        let key = minify_key(&self.msg, self.len, &self.prefix, self.threshold);
        quote! { #key }
    }
}

impl syn::parse::Parse for MiKey {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let msg = input.parse::<syn::LitStr>()?.value();
        let _comma = input.parse::<Token![,]>()?;
        let len: usize = input.parse::<syn::LitInt>()?.base10_parse()?;
        let _comma = input.parse::<Token![,]>()?;
        let prefix = input.parse::<syn::LitStr>()?.value();
        let _comma = input.parse::<Token![,]>()?;
        let threshold: usize = input.parse::<syn::LitInt>()?.base10_parse()?;
        Ok(Self {
            msg,
            len,
            prefix,
            threshold,
        })
    }
}

impl From<MiKey> for proc_macro::TokenStream {
    fn from(val: MiKey) -> Self {
        val.into_token_stream().into()
    }
}

impl From<MiKey> for proc_macro2::TokenStream {
    fn from(val: MiKey) -> Self {
        val.into_token_stream()
    }
}
