use std::{cell::RefCell, rc::Rc};

use hydroflow_plus::ir::HfPlusNode;
use quote::{quote, ToTokens};
use syn::{parse_quote, Expr, Ident, Stmt};

use crate::{
    io::IO,
    r_ast::*,
    utils::{var_name, DebugStr, Tagged},
};

// TODO: new ast? expr trait (GAT expr type), scope trait

pub trait IntoHf<'a> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a>;
}

/// Returns (return_value, scope)
impl<'a> IntoHf<'a> for RExpr<IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        match self {
            RExpr::If(s) => s.into_hf(input),
            RExpr::Block(s) => s.into_hf(input),
            RExpr::Raw(s) => s.into_hf(input),
        }
    }
}

impl<'a> IntoHf<'a> for RExprIf<IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        let Self {
            box cond_expr,
            box then_expr,
            box else_expr,
        } = self;
        let cond_node = Rc::new(RefCell::new(cond_expr.into_hf(input)));
        // TODO: Replace with partition
        // filtered for cond
        let then_cond_node = HfPlusNode::Filter {
            input: Box::new(HfPlusNode::Tee {
                inner: cond_node.clone(),
            }),
            f: cond_to_option_f().into(),
        };
        // filtered for !cond
        let else_cond_node = HfPlusNode::Filter {
            input: Box::new(HfPlusNode::Map {
                input: Box::new(HfPlusNode::Tee {
                    inner: cond_node.clone(),
                }),
                f: map_expr_f(quote! {::std::ops::Not::not}).into(),
            }),
            f: cond_to_option_f().into(),
        };

        let then_node = then_expr.into_hf(Box::new(then_cond_node));
        let else_node = else_expr.into_hf(Box::new(else_cond_node));

        HfPlusNode::Union(Box::new(then_node), Box::new(else_node))
    }
}

impl<'a> IntoHf<'a> for RExprBlock<IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        let Self {
            stmt,
            box return_expr,
        } = self;
        let stmt_node = stmt.into_hf(input).into();
        return_expr.into_hf(stmt_node)
    }
}

impl<'a> IntoHf<'a> for Tagged<DebugStr<Expr>, IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        let Tagged(
            DebugStr(expr),
            IO {
                input_scope: inputs,
                output_scope: outputs,
            },
        ) = self; // TODO: do we need outputs here?

        let f = evaluate_expr_f(expr, &inputs).into();
        HfPlusNode::Map { input, f }
    }
}

impl<'a> IntoHf<'a> for RStmt<IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        match self {
            RStmt::Let(s) => s.into_hf(input),
            RStmt::Return(s) => s.into_hf(input),
            RStmt::Expr(s) => s.into_hf(input),
        }
    }
}

impl<'a> IntoHf<'a> for Tagged<RStmtLet<IO>, IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        let Tagged(
            RStmtLet { ident, box value },
            IO {
                input_scope,
                output_scope,
            },
        ) = self;

        HfPlusNode::Map {
            input: value.into_hf(input).into(),
            f: bind_expr_f(ident, &input_scope, &output_scope).into(),
        }
    }
}

impl<'a> IntoHf<'a> for RStmtReturn<IO> {
    fn into_hf(self, input: Box<HfPlusNode<'a>>) -> HfPlusNode<'a> {
        // TODO: fix this please
        *input
    }
}

// In this section, we construct closures for nodes
// Note that scopes are tuples of variable names,
// and expressions output (return_value, scope)

/// Evalaluates an expression into a value, using some scope
/// :: scope -> (return_value, scope)
fn evaluate_expr_f(expr: Expr, scope: &[Ident]) -> Expr {
    parse_quote! {
        |#(#scope),*| (#expr, (#(#scope,)*))
    }
}

/// Evaluates a boolean for a filter_map
/// :: (condition, scope) -> Option<scope>
fn cond_to_option_f() -> Expr {
    parse_quote! {
        |(condition, scope)| condition.then_some(scope)
    }
}

/// Evaluates a map function over an expression input
/// :: (input_value, scope) -> (return_value, scope)
fn map_expr_f(map_func: impl ToTokens) -> Expr {
    parse_quote! {
        |(input_value, scope)| ((#map_func)(input_value), scope)
    }
}

/// Binds an expr return value to a variable named ident in scope
/// :: (input_value, input_scope) -> output_scope
fn bind_expr_f(ident: Ident, input_scope: &[Ident], output_scope: &[Ident]) -> Expr {
    parse_quote! {
        |(#ident, (#(#input_scope,)*))| (#(#output_scope,)*)
    }
}

//
// let x = i + 1;
// if (x < 5) {
//     let z = 1+1;
//     let y = a.await;
//     y + z
// } else {
//     let z = a.await;
//     return z;
// }
// x + 2
//
// ->
//
// input
// -> map(|i| { // raw stmt
//     let x = i + 1;
//     (x)
// })
// -> map(|(x))| { // raw expr
//     let _temp1 = x < 5;
//     (_temp1, in)
// })
// -> demux(|(_temp1, in)| {
//     _temp1 => then
//     !_temp1 => else
// })
//
// then = map(|(x))
