use std::collections::BTreeSet;

use quote::{quote, ToTokens};
use syn::Ident;

use super::functional::Semigroup;

/// Some collection of identifiers stored in a deterministic order.
/// Can be tokenized into a pattern.
#[derive(Clone, Debug)]
pub struct Scope(pub BTreeSet<Ident>);

impl Scope {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
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
