#![feature(box_patterns)]

use asta::AExpr;
use quote::quote;
use syn::{parse_quote, Expr};
mod asta;
mod parser;
mod codegen;
mod hast;
mod astb;
mod debugutil;

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

    let hex: AExpr = input.into();
    let hex_debug = format!("{:?}", hex);

    println!("fn raw() {{{}}}", hex_debug);

    let toks = quote! {
        fn main() {
            #hex
        }
    };
    // USE HYDROFLOW+ rust IR (look at IR file)

    println!("{}", toks);

    // println!("fn main() {{");
    // println!("{:?}", input);
    // println!("}}");
}


