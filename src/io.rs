use std::{collections::BTreeSet, rc::Rc};

use quote::{quote, ToTokens};
use syn::Ident;

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
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(idents) = self;
        tokens.extend(quote! {(#(#idents,)*)});
    }
}

/// Metadata wrapping a raw syn expression or statement.
/// Stores the input and output variables in a consistent order.
#[derive(Debug)]
pub struct IO {
    pub input_scope: Scope,
    pub output_scope: Scope,
}