use hydroflow_plus::ir::{HfPlusLeaf, HfPlusNode};
use syn::{Expr, Ident};

use crate::{h_ir::from_r::generate_h, hfplus_ir::from_h::generate_hf, r_ir::{ir::RExpr, scope_analysis::tag::tag}};


pub fn compile(input: Ident, hf_input: HfPlusNode, expr: Expr, debug: bool) -> (Vec<HfPlusLeaf>, HfPlusNode) {
    if debug {
        println!("fn expr(){{{:?}}}", expr);
    }
    let r_expr = RExpr::from(expr);
    if debug {
        println!("fn r_expr(){{{:?}}}", r_expr);
    }
    let r_expr_tagged = tag(r_expr, input);
    if debug {
        println!("fn r_expr_tagged(){{{:?}}}", r_expr_tagged);
    }
    let h_expr = generate_h(r_expr_tagged);
    if debug {
        println!("fn h_expr(){{{:?}}}", h_expr);
    }
    let hf = generate_hf(h_expr, hf_input);
    if debug {
        println!("fn hf(){{{:?}}}", hf);
    }
    hf
}

#[macro_export]
macro_rules! compile {
    (let $input:ident = $hf_input:expr => $($body:tt)*) => {
        crate::compile::compile(::syn::parse_quote!($input), $hf_input, ::syn::parse_quote!($($body)*), false)
    };
}

#[macro_export]
macro_rules! compile_dbg {
    (let $input:ident = $hf_input:expr => $($body:tt)*) => {
        crate::compile::compile(::syn::parse_quote!($input), $hf_input, ::syn::parse_quote!($($body)*), true)
    };
}