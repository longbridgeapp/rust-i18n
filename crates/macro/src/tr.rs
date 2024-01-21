use quote::{quote, ToTokens};
use rust_i18n_support::TrKey;
use syn::{parse::discouraged::Speculative, token::Brace, Expr, ExprMacro, Ident, LitStr, Token};

#[derive(Clone)]
pub enum Value {
    Expr(Expr),
    Ident(Ident),
}

impl From<Expr> for Value {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl From<Ident> for Value {
    fn from(ident: Ident) -> Self {
        Self::Ident(ident)
    }
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Expr(expr) => expr.to_tokens(tokens),
            Self::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let fork = input.fork();
        if let Ok(expr) = fork.parse::<Expr>() {
            input.advance_to(&fork);
            return Ok(expr.into());
        }
        let fork = input.fork();
        if let Ok(expr) = fork.parse::<Ident>() {
            input.advance_to(&fork);
            return Ok(expr.into());
        }
        Err(input.error("Expected a expression or an identifier"))
    }
}

pub struct Argument {
    pub name: String,
    pub value: Value,
    pub specifiers: Option<String>,
}

impl Argument {
    #[allow(dead_code)]
    pub fn value_string(&self) -> String {
        match &self.value {
            Value::Expr(Expr::Lit(expr_lit)) => match &expr_lit.lit {
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
        if input.peek(Token![=>]) {
            let _ = input.parse::<Token![=>]>()?;
        } else if input.peek(Token![=]) {
            let _ = input.parse::<Token![=]>()?;
        } else {
            return Err(input.error("Expected `=>` or `=`"));
        }
        let value = input.parse()?;
        let specifiers = if input.peek(Token![:]) {
            let _ = input.parse::<Token![:]>()?;
            if input.peek(Brace) {
                let content;
                let _ = syn::braced!(content in input);
                let mut specifiers = String::new();
                while let Ok(s) = content.parse::<proc_macro2::TokenTree>() {
                    specifiers.push_str(&s.to_string());
                }
                Some(specifiers)
            } else {
                None
            }
        } else {
            None
        };
        Ok(Self {
            name,
            value,
            specifiers,
        })
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

    #[allow(dead_code)]
    pub fn values(&self) -> Vec<Value> {
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
    pub locale: Option<Value>,
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
            .as_ref()
            .iter()
            .map(|v| {
                let value = &v.value;
                let sepecifiers = v
                    .specifiers
                    .as_ref()
                    .map_or("{}".to_owned(), |s| format!("{{{}}}", s));
                quote! { format!(#sepecifiers, #value) }
            })
            .collect();
        let logging = if cfg!(feature = "log-tr-dyn") {
            quote! {
                log::warn!("tr: missing: {} => {:?} @ {}:{}", msg_key, msg_val, file!(), line!());
            }
        } else {
            quote! {}
        };
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
                if self.args.is_empty() {
                    quote! {
                        {
                            let msg_key = rust_i18n::TrKey::tr_key(&#msg_key);
                            let msg_val = #msg_val;
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, &msg_key) {
                                translated
                            } else {
                                #logging
                                rust_i18n::CowStr::from(msg_val).into_inner()
                            }
                        }
                    }
                } else {
                    quote! {
                        {
                            let msg_key = rust_i18n::TrKey::tr_key(&#msg_key);
                            let msg_val = #msg_val;
                            let keys = &[#(#keys),*];
                            let values = &[#(#values),*];
                            if let Some(translated) = crate::_rust_i18n_try_translate(#locale, &msg_key) {
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
