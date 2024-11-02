use proc_macro2::TokenStream;
use syn::{parse_quote, Expr};

use crate::utils::pattern::Pattern;

// Utilities for constructing function closures

// :: I -> O
pub struct MapFunc<I, O> {
    ins: I,
    outs: O,
    body: TokenStream,
}

// :: I -> Option<O>
pub struct FilterMapFunc<I, O> {
    ins: I,
    outs: O,
    body: TokenStream,
}

impl<I, O> MapFunc<I, O> {
    pub fn newb(ins: I, outs: O, body: TokenStream) -> Self {
        Self { ins, outs, body }
    }

    pub fn new(ins: I, outs: O) -> Self {
        Self::newb(ins, outs, TokenStream::new())
    }
}

impl<I, O> FilterMapFunc<I, O> {
    pub fn newb(ins: I, outs: O, body: TokenStream) -> Self {
        Self { ins, outs, body }
    }

    pub fn new(ins: I, outs: O) -> Self {
        Self::newb(ins, outs, TokenStream::new())
    }
}

impl<I: Pattern, O: Pattern> From<MapFunc<I, O>> for Expr {
    fn from(MapFunc { ins, outs, body }: MapFunc<I, O>) -> Self {
        parse_quote! {
            |#ins|
            {
                #body
                #outs
            }
        }
    }
}

impl<I: Pattern, O: Pattern> From<FilterMapFunc<I, O>> for Expr {
    fn from(FilterMapFunc { ins, outs, body }: FilterMapFunc<I, O>) -> Self {
        parse_quote! {
            |#ins|
            {
                #body
                Some(#outs)
            }
        }
    }
}