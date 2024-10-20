#![feature(box_patterns)]

use code_gen::IntoHf;
use hydroflow_plus::ir::HfPlusNode;
use r_ast::RExpr;
use syn::{parse_quote, Expr};
mod r_ast;
mod parser;
mod code_gen;
mod hast;
mod utils;
mod scope_analysis;
mod io;

//steps:
// 1) Encapsulate special-cased rust logic, pulling all dataflow-relevant operations above the barrier
// 2) Traverse tree backwards, annotating all scoping information
// 3) Produce hydroflow from the annotated tree

//old
// 1) transform into subset of rust with simple statements and expressions
// 2) transform these statements into dataflowy structure?
// 3) output hydroflow

pub fn main() {
    let input: Expr = parse_quote! {
        {
            let x = 1 + 1;
            if (x < 5) {
                let z = 1+1;
                let y = a.await;
                y + z
            } else if (x > 10) {
                let z = a.await;
                z + x
            } else {
                let z = a.await;
                return z;
            }
            x + 2
        }
    };

    let rex = RExpr::from(input);

    println!("fn raw() {{{:?}}}", rex);

    let rex_tagged = rex.tag();

    println!("fn tagged() {{{:?}}}", rex_tagged);

    let hex = rex_tagged.into_hf(Box::new(HfPlusNode::Placeholder));

    println!("fn hydroflow() {{{:?}}}", hex);


    // println!("fn main() {{");
    // println!("{:?}", input);
    // println!("}}");
}


