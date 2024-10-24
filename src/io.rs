use std::{collections::BTreeSet, rc::Rc};

use quote::{quote, ToTokens};
use syn::Ident;

/// Some collection of identifiers stored in a deterministic order.
/// Can be tokenized into a pattern.
#[derive(Clone, Debug)]
pub struct Scope(pub BTreeSet<Ident>);

impl Scope {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
    }

    pub fn with(&self, ident: Ident) -> Self {
        let mut idents = self.0.clone();
        idents.insert(ident);
        Self(idents)
    }

    pub fn without(&self, ident: Ident) -> Self {
        let mut idents = self.0.clone();
        idents.remove(&ident);
        Self(idents)
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(idents) = self;
        tokens.extend(quote! {(#(#idents,)*)});
    }
}

/// Metadata wrapping a raw syn expression or a binding.
#[derive(Debug, Clone)]
pub struct IO {
    pub ins: Scope,
    pub outs: Scope,
}
