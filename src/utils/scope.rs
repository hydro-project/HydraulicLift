use std::collections::BTreeSet;

use quote::{quote, ToTokens};
use syn::Ident;

use super::functional::Semigroup;

/// Some collection of identifiers and attributes (mut TODO) stored in a deterministic order.
/// Can be tokenized into a pattern.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope(pub BTreeSet<Ident>);


impl Scope {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn with(self, ident: Ident) -> Self {
        let Self(mut idents) = self;
        idents.insert(ident);
        Self(idents)
    }

    pub fn without(self, ident: &Ident) -> Self {
        let Self(mut idents) = self;
        idents.remove(ident);
        Self(idents)
    }

    pub fn intersect(self, other: Self) -> Self {
        let mut new = BTreeSet::new();
        new.extend(self.0.intersection(&other.0).map(|x| (*x).clone()));
        Self(new)
    }
}

impl Semigroup for Scope {
    fn concat(self, Self(mut inner): Self) -> Self {
        inner.extend(self.0);
        Self(inner)
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(idents) = self;
        tokens.extend(quote! {(#(#idents),*)});
    }
}