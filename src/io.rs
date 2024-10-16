use std::rc::Rc;

use syn::Ident;

/// Metadata wrapping a raw syn expression or statement.
/// Stores the input and output variables in a consistent order.
pub struct IO {
    pub inputs: Vec<Ident>,
    pub outputs: Vec<Ident>,
}