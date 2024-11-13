use std::collections::{BTreeMap};

use quote::{quote, ToTokens};
use syn::Ident;

use super::functional::Semigroup;

/// Scope with mutability tag
pub type ScopeDef = Scope<bool>;

/// Some collection of identifiers and attributes (M) stored in a deterministic order.
/// Can be tokenized into a pattern.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope<M=()>(pub BTreeMap<Ident, M>);

impl<M> Scope<M> {
    pub fn empty() -> Self {
        Self(BTreeMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn with(mut self, ident: Ident, m: M) -> Self {
        self.0.insert(ident, m);
        self
    }

    pub fn without(mut self, ident: &Ident) -> Self {
        self.0.remove(ident);
        self
    }

    /// Intersects with other, preserving self's metadata
    pub fn intersect(mut self, other: Scope) -> Self {
        self.0.retain(|k, _| other.0.contains_key(k));
        self
    }

    /// Drops metadata on self
    pub fn lower(self) -> Scope {
        Scope(self.0.into_iter().map(|(ident, _)|(ident, ())).collect())
    }
}

impl Scope {
    pub fn wth(self, ident: Ident) -> Self {
        self.with(ident, ())
    }
}

impl<M> Semigroup for Scope<M> {
    fn concat(self, Self(mut inner): Self) -> Self {
        inner.extend(self.0);
        Self(inner)
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let idents = self.0.keys();
        tokens.extend(quote! {(#(#idents),*)});
    }
}

impl ToTokens for ScopeDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let elems = self.0.iter().map(|(ident, is_mut)| IdPat(ident, is_mut));
        tokens.extend(quote! {(#(#elems),*)});
    }
}

struct IdPat<'a>(&'a Ident, &'a bool);

impl<'a> ToTokens for IdPat<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if *self.1 {
            tokens.extend(quote! {mut});
        }
        tokens.extend(self.0.to_token_stream());
    }
}

