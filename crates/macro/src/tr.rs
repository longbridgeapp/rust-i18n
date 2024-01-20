use quote::{quote, ToTokens};
use rust_i18n_support::TrKey;
use syn::{parse::discouraged::Speculative, Expr, ExprMacro, Ident, LitStr, Token};

pub struct Argument {
    pub name: String,
    pub value: Expr,
}

impl Argument {
    #[allow(dead_code)]
    pub fn value_string(&self) -> String {
        match &self.value {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(lit_str) => lit_str.value(),
                _ => self.value.to_token_stream().to_string(),
            },
            _ => self.value.to_token_stream().to_string(),
        }
    }
}

impl syn::parse::Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name = input
            .parse::<Ident>()
            .map(|v| v.to_string())
            .or_else(|_| input.parse::<LitStr>().map(|v| v.value()))
            .map_err(|_| input.error("Expected a `string` literal or an identifier"))?;
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Option<Token![>]>>()?;
        let value = input.parse::<Expr>()?;
        Ok(Self { name, value })
    }
}

#[derive(Default)]
pub struct Arguments {
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn keys(&self) -> Vec<String> {
        self.args.iter().map(|arg| arg.name.clone()).collect()
    }

    pub fn values(&self) -> Vec<Expr> {
        self.args.iter().map(|arg| arg.value.clone()).collect()
    }
}

impl AsRef<Vec<Argument>> for Arguments {
    fn as_ref(&self) -> &Vec<Argument> {
        &self.args
    }
}

impl AsMut<Vec<Argument>> for Arguments {
    fn as_mut(&mut self) -> &mut Vec<Argument> {
        &mut self.args
    }
}

impl syn::parse::Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let args = input
            .parse_terminated(Argument::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self { args })
    }
}

#[derive(Default)]
pub enum Messagekind {
    #[default]
    Literal,
    Expr,
    ExprCall,
    ExprClosure,
    ExprMacro,
    ExprReference,
    ExprUnary,
    Ident,
}

#[derive(Default)]
pub struct Messsage {
    pub key: proc_macro2::TokenStream,
    pub val: proc_macro2::TokenStream,
    pub kind: Messagekind,
}

impl Messsage {
    fn try_exp(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let fork = input.fork();
        let expr = fork.parse::<Expr>()?;
        let key = quote! { #expr };
        let val = quote! { #expr };
        input.advance_to(&fork);
        let kind = match expr {
            Expr::Call(_) => Messagekind::ExprCall,
            Expr::Closure(_) => Messagekind::ExprClosure,
            Expr::Macro(_) => Messagekind::ExprMacro,
            Expr::Reference(_) => Messagekind::ExprReference,
            Expr::Unary(_) => Messagekind::ExprUnary,
            _ => Messagekind::Expr,
        };
        Ok(Self { key, val, kind })
    }

    #[allow(dead_code)]
    fn try_exp_macro(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let fork = input.fork();
        let expr = fork.parse::<ExprMacro>()?;
        let key = quote! { #expr };
        let val = quote! { #expr };
        input.advance_to(&fork);
        Ok(Self {
            key,
            val,
            kind: Messagekind::ExprMacro,
        })
    }

    fn try_ident(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let fork = input.fork();
        let ident = fork.parse::<Ident>()?;
        let key = quote! { #ident };
        let val = quote! { #ident };
        input.advance_to(&fork);
        Ok(Self {
            key,
            val,
            kind: Messagekind::Ident,
        })
    }

    fn try_litreal(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let fork = input.fork();
        let lit_str = fork.parse::<LitStr>()?;
        let key = lit_str.value().tr_key();
        let key = quote! { #key };
        let val = quote! { #lit_str };
        input.advance_to(&fork);
        Ok(Self {
            key,
            val,
            kind: Messagekind::Literal,
        })
    }
}

impl syn::parse::Parse for Messsage {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let result = Self::try_litreal(input)
            // .or_else(|_| Self::try_exp_macro(input))
            .or_else(|_| Self::try_exp(input))
            .or_else(|_| Self::try_ident(input))?;
        Ok(result)
    }
}

/// A type representing the `tr!` macro.
#[derive(Default)]
pub(crate) struct Tr {
    pub msg: Messsage,
    pub args: Arguments,
    pub locale: Option<Expr>,
}

impl Tr {
    fn into_token_stream(self) -> proc_macro2::TokenStream {
        let msg_key = self.msg.key;
        let msg_val = self.msg.val;
        let msg_kind = self.msg.kind;
        let locale = self.locale.map_or_else(
            || quote! { &rust_i18n::locale() },
            |locale| quote! { #locale },
        );
        let keys: Vec<_> = self.args.keys().iter().map(|v| quote! { #v }).collect();
        let values: Vec<_> = self
            .args
            .values()
            .iter()
            .map(|v| quote! { format!("{}", #v) })
            .collect();
        match msg_kind {
            Messagekind::Literal => {
                if self.args.is_empty() {
                    quote! {
                        crate::_rust_i18n_try_translate(#locale, #msg_key).unwrap_or_else(|| std::borrow::Cow::from(#msg_val))
                    }
                } else {
                    quote! {
                        {
                            let msg_key = #msg_key;
                            let msg_val = #msg_val;
                            let keys = &[#(#keys),*];
                            let values = &[#(#values),*];
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, &msg_key) {
                                let replaced = rust_i18n::replace_patterns(&translated, keys, values);
                                std::borrow::Cow::from(replaced)
                            } else {
                                let replaced = rust_i18n::replace_patterns(&rust_i18n::CowStr::from(msg_val).into_inner(), keys, values);
                                std::borrow::Cow::from(replaced)
                            }
                        }
                    }
                }
            }
            Messagekind::ExprCall | Messagekind::ExprClosure | Messagekind::ExprMacro => {
                let logging = if cfg!(feature = "log_tr_dyn") {
                    quote! {
                        log::debug!("tr: missing: {} => {:?} @ {}:{}", msg_key, msg_val, file!(), line!());
                    }
                } else {
                    quote! {}
                };
                if self.args.is_empty() {
                    quote! {
                        {
                            let msg_val = #msg_val;
                            let msg_key = rust_i18n::TrKey::tr_key(&msg_val);
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, msg_key) {
                                translated
                            } else {
                                #logging
                                std::borrow::Cow::from(msg_val)
                            }
                        }
                    }
                } else {
                    quote! {
                        {
                            let msg_val = #msg_val;
                            let msg_key = rust_i18n::TrKey::tr_key(&msg_val);
                            let keys = &[#(#keys),*];
                            let values = &[#(#values),*];
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, msg_key) {
                                let replaced = rust_i18n::replace_patterns(&translated, keys, values);
                                std::borrow::Cow::from(replaced)
                            } else {
                                #logging
                                let replaced = rust_i18n::replace_patterns(&rust_i18n::CowStr::from(msg_val).into_inner(), keys, values);
                                std::borrow::Cow::from(replaced)
                            }
                        }
                    }
                }
            }
            Messagekind::Expr
            | Messagekind::ExprReference
            | Messagekind::ExprUnary
            | Messagekind::Ident => {
                let logging = if cfg!(feature = "log_tr_dyn") {
                    quote! {
                        log::debug!("tr: missing: {} => {:?} @ {}:{}", msg_key, #msg_val, file!(), line!());
                    }
                } else {
                    quote! {}
                };
                if self.args.is_empty() {
                    quote! {
                        {
                            let msg_key = rust_i18n::TrKey::tr_key(&#msg_key);
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, &msg_key) {
                                translated
                            } else {
                                rust_i18n::CowStr::from(#msg_val).into_inner()
                            }
                        }
                    }
                } else {
                    quote! {
                        {
                            let msg_key = rust_i18n::TrKey::tr_key(&#msg_key);
                            let keys = &[#(#keys),*];
                            let values = &[#(#values),*];
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, &msg_key) {
                                let replaced = rust_i18n::replace_patterns(&translated, keys, values);
                                std::borrow::Cow::from(replaced)
                            } else {
                                #logging
                                let replaced = rust_i18n::replace_patterns(&rust_i18n::CowStr::from(#msg_val).into_inner(), keys, values);
                                std::borrow::Cow::from(replaced)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl syn::parse::Parse for Tr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let msg = input.parse::<Messsage>()?;
        let comma = input.parse::<Option<Token![,]>>()?;
        let (args, locale) = if comma.is_some() {
            let mut args = input.parse::<Arguments>()?;
            let locale = args
                .as_ref()
                .iter()
                .find(|v| v.name == "locale")
                .map(|v| v.value.clone());
            args.as_mut().retain(|v| v.name != "locale");
            (args, locale)
        } else {
            (Arguments::default(), None)
        };

        Ok(Self { msg, args, locale })
    }
}

impl From<Tr> for proc_macro::TokenStream {
    fn from(args: Tr) -> Self {
        args.into_token_stream().into()
    }
}

impl From<Tr> for proc_macro2::TokenStream {
    fn from(args: Tr) -> Self {
        args.into_token_stream()
    }
}
