use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use super::scope::{Scope, ScopeDef};

/// Special case of ToTokens which confirms that the tokenized version is a pattern
/// TODO: add as_input vs as_output tokenization
pub trait Pattern: ToTokens {}


impl Pattern for Ident {}

/// matches: scope | (a, b, c) | (a, mut b, c)
pub enum ScopePat {
    Ident(Ident),
    Destructured(Scope),
    DestructuredDef(ScopeDef),
}
impl Pattern for ScopePat {}

/// matches: expr | (value, scope) | (a, (b, c, d))
pub enum ExprPat {
    Ident(Ident),
    Destructured(Ident, ScopePat),
}
impl Pattern for ExprPat {}

impl ToTokens for ScopePat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Destructured(scope) => scope.to_tokens(tokens),
            Self::DestructuredDef(scope) => scope.to_tokens(tokens),
        };
    }
}

impl ToTokens for ExprPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Destructured(ident, scope_pat) => tokens.extend(quote! {(#ident, #scope_pat)}),
        }
    }
}





