use std::fmt::Debug;

use syn::{Expr, Ident};

use crate::utils::{debug::DebugStr, tagged::TagOut};

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust


// I/O are tagged metadata corresponding to the inputs or outputs of a node.

#[derive(Debug, Clone)]
pub enum RExpr<I = (), O = ()> {
    If(RExprIf<I, O>),
    Block(RExprBlock<I, O>),
    Await(RExprAwait<I, O>),
    Raw(TagOut<RExprRaw<I>, O>), //TODO: expand
}

#[derive(Debug, Clone)]
pub struct RExprIf<I = (), O = ()> {
    pub cond_expr: Box<RExpr<I, O>>,
    pub then_expr: Box<RExpr<I, O>>,
    pub else_expr: Box<RExpr<I, O>>,
}

#[derive(Debug, Clone)]
pub struct RExprBlock<I = (), O = ()> {
    pub stmt: RStmt<I, O>,
    pub expr: Box<RExpr<I, O>>,
}

#[derive(Debug, Clone)]
pub struct RExprAwait<I = (), O = ()>(pub Box<RExpr<I, O>>);

#[derive(Debug, Clone)]
pub struct RExprRaw<I = ()> {
    pub expr: DebugStr<Expr>,
    pub scope: I,
}

#[derive(Debug, Clone)]
pub enum RStmt<I = (), O = ()> {
    Let(TagOut<RStmtLet<I, O>, O>),
    Return(RStmtReturn<I, O>),
    While(RStmtWhile<I, O>)
    // TODO: add expressions here?
}

#[derive(Debug, Clone)]
pub struct RStmtLet<I = (), O = ()> {
    pub id: Ident,
    pub is_mut: bool,
    pub value: Box<RExpr<I, O>>,
}

#[derive(Debug, Clone)]
pub struct RStmtReturn<I = (), O = ()> {
    pub value: Box<RExpr<I, O>>,
}

#[derive(Debug, Clone)]
pub struct RStmtWhile<I = (), O = ()> {
    pub cond: Box<RExpr<I, O>>,
    pub body: Box<RExpr<I, O>>,
}

impl<I, O> RExprIf<I, O> {
    pub fn new(cond_expr: RExpr<I, O>, then_expr: RExpr<I, O>, else_expr: RExpr<I, O>) -> Self {
        Self {
            cond_expr: Box::new(cond_expr),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        }
    }
}

impl<I, O> RExprBlock<I, O> {
    pub fn new(stmt: RStmt<I, O>, expr: RExpr<I, O>) -> Self {
        Self {
            stmt,
            expr: Box::new(expr),
        }
    }
}

impl<I, O> RExprAwait<I, O> {
    pub fn new(expr: RExpr<I, O>) -> Self {
        Self(Box::new(expr))
    }
}

impl<I> RExprRaw<I> {
    pub fn new(expr: Expr, input_scope: I) -> Self {
        Self {
            expr: expr.into(),
            scope: input_scope,
        }
    }
}

impl<I, O> RStmtLet<I, O> {
    pub fn new(id: Ident, is_mut: bool, value: RExpr<I, O>) -> Self {
        Self {
            id,
            is_mut,
            value: Box::new(value),
        }
    }
}

impl<I, O> RStmtReturn<I, O> {
    pub fn new(value: RExpr<I, O>) -> Self {
        Self {
            value: Box::new(value),
        }
    }
}

impl<I, O> RStmtWhile<I, O> {
    pub fn new(cond: RExpr<I, O>, body: RExpr<I, O>) -> Self {
        Self {
            cond: Box::new(cond),
            body: Box::new(body),
        }
    }
}

