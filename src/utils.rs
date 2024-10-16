use std::fmt::Debug;


use proc_macro2::TokenStream;

use quote::ToTokens;

/// Wrapping a raw structure with some metadata
#[derive(Debug)]
pub struct Tagged<T, M>(pub T, pub M);


/// Wraps a type, replaces the debug view with a string view
pub struct DebugStr<T>(pub T);

impl<T: ToTokens> Debug for DebugStr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}", self.0.to_token_stream());
        f.debug_struct("DebugStr").field("inner", &s).finish()
    }
}

impl<T> From<T> for DebugStr<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}