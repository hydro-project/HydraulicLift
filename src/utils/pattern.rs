use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use super::scope::Scope;

/// Special case of ToTokens which confirms that the tokenized version is a pattern
/// TODO: add as_input vs as_output tokenization
pub trait Pattern: ToTokens {}


impl Pattern for Ident {}

/// matches: scope | (a, b, c)
pub enum ScopePat {
    Ident(Ident),
    Destructured(Scope),
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
            ScopePat::Ident(ident) => ident.to_tokens(tokens),
            ScopePat::Destructured(scope) => scope.to_tokens(tokens),
        };
    }
}

impl ToTokens for ExprPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprPat::Ident(ident) => ident.to_tokens(tokens),
            ExprPat::Destructured(ident, scope_pat) => tokens.extend(quote! {(#ident, #scope_pat)}),
        }
    }
}





