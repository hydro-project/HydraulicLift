use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro2::Span;
use syn::Ident;

/// Todo: make this better and not constant lol
pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

/// Returns a globally unique ident
pub fn unique_ident() -> Ident {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let x = COUNTER.fetch_add(1, Ordering::Relaxed);

    ident(&format!("__HYDRAULIC_LIFT__{x}"))
}