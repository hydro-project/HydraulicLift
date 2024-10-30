use hydroflow_plus::ir::HfPlusNode;
use syn::{Expr, Ident};

use crate::{r_ir::ir::RExpr, utils::scope::Scope};


pub fn compile(input: Ident, hf_input: HfPlusNode, expr: Expr, debug: bool) -> HfPlusNode {
    if debug {
        println!("fn expr(){{{:?}}}", expr);
    }
    let r_expr = RExpr::from(expr);
    if debug {
        println!("fn r_expr(){{{:?}}}", r_expr);
    }
    let r_expr_tagged = RExpr::<Scope>::from(r_expr);
    if debug {
        println!("fn r_expr_tagged(){{{:?}}}", r_expr_tagged);
    }
    let h_expr = HOutput::from(r_expr_tagged);
    if debug {
        println!("fn h_expr(){{{:?}}}", h_expr);
    }
    let hf = generate(h_expr, hf_input);
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