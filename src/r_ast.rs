use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Ident, Stmt};

use crate::utils::{DebugStr, Tagged};

// TODO - Make tagged ast

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug, Clone)]
pub enum RExpr<M = ()> {
    If(RExprIf<M>),
    Block(RExprBlock<M>),
    //TODO: add await
    Raw(Tagged<RExprRaw, M>), //TODO: expand
}

#[derive(Debug, Clone)]
pub struct RExprIf<M = ()> {
    pub cond_expr: Box<RExpr<M>>,
    pub then_expr: Box<RExpr<M>>,
    pub else_expr: Box<RExpr<M>>,
}

/// sequence of statements which evaluates to a value
#[derive(Debug, Clone)]
pub struct RExprBlock<M = ()> {
    pub stmt: RStmt<M>,
    pub expr: Box<RExpr<M>>,
}

#[derive(Debug, Clone)]
pub struct RExprRaw(pub DebugStr<Expr>);

#[derive(Debug, Clone)]
pub enum RStmt<M = ()> {
    Let(Tagged<RStmtLet<M>, M>),
    Return(RStmtReturn<M>),
    // TODO: add expressions here?
}

#[derive(Debug, Clone)]
pub struct RStmtLet<M = ()> {
    pub id: Ident,            // y
    pub value: Box<RExpr<M>>, // x
}

// derive debug expression
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

impl RExprRaw {
    pub fn new(expr: Expr) -> Self {
        Self(expr.into())
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
