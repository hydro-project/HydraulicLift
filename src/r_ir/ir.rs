use std::fmt::Debug;

use syn::{Expr, Ident};

use crate::utils::{debug::DebugStr, tagged::Tagged};

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug, Clone)]
pub enum RExpr<M = ()> {
    If(RExprIf<M>),
    Block(RExprBlock<M>),
    Await(RExprAwait<M>),
    Raw(Tagged<RExprRaw<M>, M>), //TODO: expand
}

#[derive(Debug, Clone)]
pub struct RExprIf<M = ()> {
    pub cond_expr: Box<RExpr<M>>,
    pub then_expr: Box<RExpr<M>>,
    pub else_expr: Box<RExpr<M>>,
}

#[derive(Debug, Clone)]
pub struct RExprBlock<M = ()> {
    pub stmt: RStmt<M>,
    pub expr: Box<RExpr<M>>,
}

#[derive(Debug, Clone)]
pub struct RExprAwait<M = ()>(pub Box<RExpr<M>>);

#[derive(Debug, Clone)]
pub struct RExprRaw<M = ()> {
    pub expr: DebugStr<Expr>,
    pub scope: M,
}

#[derive(Debug, Clone)]
pub enum RStmt<M = ()> {
    Let(Tagged<RStmtLet<M>, M>),
    Return(RStmtReturn<M>),
    // TODO: add expressions here?
}

#[derive(Debug, Clone)]
pub struct RStmtLet<M = ()> {
    pub id: Ident,
    pub value: Box<RExpr<M>>,
}

#[derive(Debug, Clone)]
pub struct RStmtReturn<M = ()> {
    pub value: Box<RExpr<M>>,
}

impl<M> RExprIf<M> {
    pub fn new(cond_expr: RExpr<M>, then_expr: RExpr<M>, else_expr: RExpr<M>) -> Self {
        Self {
            cond_expr: Box::new(cond_expr),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        }
    }
}

impl<M> RExprBlock<M> {
    pub fn new(stmt: RStmt<M>, expr: RExpr<M>) -> Self {
        Self {
            stmt,
            expr: Box::new(expr),
        }
    }
}

impl<M> RExprAwait<M> {
    pub fn new(expr: RExpr<M>) -> Self {
        Self(Box::new(expr))
    }
}

impl<M> RExprRaw<M> {
    pub fn new(expr: Expr, input_scope: M) -> Self {
        Self {
            expr: expr.into(),
            scope: input_scope,
        }
    }
}

impl<M> RStmtLet<M> {
    pub fn new(id: Ident, value: RExpr<M>) -> Self {
        Self {
            id,
            value: Box::new(value),
        }
    }
}

impl<M> RStmtReturn<M> {
    pub fn new(value: RExpr<M>) -> Self {
        Self {
            value: Box::new(value),
        }
    }
}
