use std::rc::Rc;

use syn::Ident;

/// Metadata wrapping a raw syn expression or statement.
/// Stores the input and output variables in a consistent order.
#[derive(Debug)]
pub struct IO {
    pub input_scope: Vec<Ident>,
    pub output_scope: Vec<Ident>,
}