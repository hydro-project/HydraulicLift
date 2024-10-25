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
pub struct Tagged<T, M=()>(pub T, pub M);

impl<T> From<T> for Tagged<T, ()> {
    fn from(value: T) -> Self {
        Self(value, ())
    }
}

/// Wraps a type, replaces the debug view with the totoks view
#[derive(Clone)]
pub struct DebugStr<T: ToTokens>(pub T);

impl<T: ToTokens> ToTokens for DebugStr<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl<T: ToTokens> Debug for DebugStr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_token_stream())
    }
}

impl<T: ToTokens> From<T> for DebugStr<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: ToTokens> Deref for DebugStr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ToTokens> From<T> for Tagged<DebugStr<T>, ()> {
    fn from(value: T) -> Self {
        Self::from(DebugStr::from(value))
    }
}

/// No higher kinded types :(
pub trait TupleFunctor {
    type T;
    type With<U>;
    fn map<F, U>(self, f: F) -> Self::With<U> where F: FnOnce(Self::T) -> U ;
}

impl<X, T> TupleFunctor for (X, T) {
    type T=T;
    type With<U> = (X, U);
    fn map<F, U>(self, f: F) -> Self::With<U> where F: FnOnce(Self::T) -> U {
        (self.0, f(self.1))
    }
}