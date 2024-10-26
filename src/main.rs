#![feature(box_patterns)]

use std::collections::BTreeSet;

use code_gen::generate;
use hydroflow_plus::ir::HfPlusNode;
use io::{Scope, IO};
use ir2::HOutput;
use r_ast::RExpr;
use syn::{parse_quote, Expr};
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

//steps:
// 1) Encapsulate special-cased rust logic, pulling all dataflow-relevant operations above the barrier
// 2) Traverse tree backwards, annotating all scoping information
// 3) Produce hydroflow from the annotated tree

//old
// 1) transform into subset of rust with simple statements and expressions
// 2) transform these statements into dataflowy structure?
// 3) output hydroflow

pub fn main() {
    return test();
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

    println!("fn R() {{{:?}}}\n\n\n", rex);

    let rex_tagged = RExpr::<Scope>::from(rex);

    println!("fn R_tag() {{{:?}}}\n\n\n", rex_tagged);

    let hex = HOutput::from(rex_tagged);

    println!("fn H() {{{:?}}}\n\n\n", hex);

    let hf = generate(hex, HfPlusNode::Placeholder);

    println!("fn HFPlus() {{{:?}}}", hf)

    // // println!("fn main() {{");
    // // println!("{:?}", input);
    // // println!("}}");
}

macro_rules! compile {
    (let $input:ident = $hf_input:expr => $($body:tt)*) => {
        compile!(false, let $input = $hf_input => $($body)*)
    };
    (debug let $input:ident = $hf_input:expr => $($body:tt)*) => {
        compile!(true, let $input = $hf_input => $($body)*)
    };
    ($debug:expr, let $input:ident = $hf_input:expr => $($body:tt)*) => {
        {
            let expr: ::syn::Expr = parse_quote! {
                $($body)*
            };
            println!("fn input(){{{}}}", quote::quote!(#expr));
            if $debug { println!("fn expr(){{{:?}}}", expr); }
            let r_expr = RExpr::from(expr);
            if $debug { println!("fn r_expr(){{{:?}}}", r_expr); }
            let r_expr_tagged = RExpr::<Scope>::from(r_expr);
            if $debug { println!("fn r_expr_tagged(){{{:?}}}", r_expr_tagged); }
            let h_expr = HOutput::from(r_expr_tagged);
            if $debug { println!("fn h_expr(){{{:?}}}", h_expr); }
            let hf = generate(h_expr, $hf_input);
            if $debug { println!("fn hf(){{{:?}}}", hf); }
            hf
        }
    };
}


fn test() {
    let hf = compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = hf_in + 1;
        return x;
        x + 2 // this doesn't show up in the resulting HF+!
    });

    println!("fn HFPlus() {{{}\n}}", visualize(hf));
}
