#![feature(box_patterns)]

use std::collections::BTreeSet;

use code_gen::generate;
use hydroflow_plus::ir::HfPlusNode;
use io::{Scope, IO};
use ir2::HOutput;
use r_ast::RExpr;
use syn::{parse_quote, Expr, Ident};
use utils::ident;
use visualize::visualize;
mod code_gen;
mod hast;
mod io;
mod ir;
mod ir2;
mod parser;
mod r_ast;
mod scope_analysis;
mod transform;
mod utils;
mod visualize;
mod compile;
mod tests;

//steps:
// 1) Encapsulate special-cased rust logic, pulling all dataflow-relevant operations above the barrier
// 2) Traverse tree backwards, annotating all scoping information
// 3) Produce hydroflow from the annotated tree

//old
// 1) transform into subset of rust with simple statements and expressions
// 2) transform these statements into dataflowy structure?
// 3) output hydroflow

pub fn main() {
    let hf = compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = hf_in + 1;
        return x;
        x + 2 // this doesn't show up in the resulting HF+!
    });

    println!("fn HFPlus() {{{}\n}}", visualize(hf));
}
