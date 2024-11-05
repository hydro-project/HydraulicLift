use syn::Ident;

use crate::{r_ir::ir::RExpr, utils::{scope::Scope, tagged::Tagged}};

use super::{tag_in::tag_possible_inputs, tag_out::tag_outputs};

/// Tags nodes with their output scopes by traversing backwards 
/// and only marking needed outputs which are possible inputs.
pub fn tag(untagged: RExpr, ident: Ident) -> Tagged<RExpr<Scope>, Scope> {
    let input = Scope::empty().with(ident);
    let input_tagged = tag_possible_inputs(untagged, input);
    tag_outputs(input_tagged)
}