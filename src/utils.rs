use std::{fmt::Debug, ops::Deref};


use proc_macro2::{Span, TokenStream};

use quote::ToTokens;
use syn::Ident;


/// Todo: make this better and not constant lol
pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

/// Wrapping a raw structure with some metadata
#[derive(Debug, Clone)]
pub struct Tagged<T, M>(pub T, pub M);

impl<T> From<T> for Tagged<T, ()> {
    fn from(value: T) -> Self {
        Self(value, ())
    }
}


/// Wraps a type, replaces the debug view with a string view
#[derive(Clone)]
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

impl<T> Deref for DebugStr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<T> From<T> for Tagged<DebugStr<T>, ()> {
    fn from(value: T) -> Self {
        Self::from(DebugStr::from(value))
    }
}