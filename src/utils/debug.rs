use std::{fmt::Debug, ops::Deref};

use proc_macro2::TokenStream;
use quote::ToTokens;

/// Wraps a type, replaces the debug view with the totoks view
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DebugStr<T>(pub T);

impl<T: ToTokens> ToTokens for DebugStr<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> Debug for DebugStr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_token_stream())
    }
}

impl<T> From<T> for DebugStr<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Deref for DebugStr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}