use syn::Ident;

use crate::{r_ir::ir::RExpr, utils::{scope::{ScopeDef, Scope}, tagged::TagOut}};

use super::{tag_in::tag_possible_inputs, tag_out::tag_outputs};

/// Tags nodes with their output scopes by traversing backwards 
/// and only marking needed outputs which are possible inputs.
pub fn tag(untagged: RExpr, ident: Ident) -> TagOut<RExpr<ScopeDef, Scope>, Scope> {
    // Forwards pass to identify all possible inputs
    let input_tagged = tag_possible_inputs(untagged,  Scope::empty().with(ident, true));
    // Backwards pass to identify needed outputs
    tag_outputs(input_tagged)
}