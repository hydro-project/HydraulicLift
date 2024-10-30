use proc_macro2::Span;
use syn::Ident;

/// Todo: make this better and not constant lol
pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}