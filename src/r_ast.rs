use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Ident, Stmt};

use crate::utils::{DebugStr, Tagged};

// TODO - Make tagged ast


/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug, Clone)]
pub enum RExpr<M> {
    If(RExprIf<M>),
    Block(RExprBlock<M>),
    //TODO: add await
    Raw(Tagged<DebugStr<Expr>, M>), //TODO: expand
}

#[derive(Debug, Clone)]
pub struct RExprIf<M> {
    pub cond_expr: Box<RExpr<M>>,
    pub then_expr: Box<RExpr<M>>,
    pub else_expr: Box<RExpr<M>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug, Clone)]
pub struct RExprBlock<M> {
    pub stmt: RStmt<M>,
    pub return_expr: Box<RExpr<M>>
}

#[derive(Debug, Clone)]
pub enum RStmt<M> {
    Let(Tagged<RStmtLet<M>, M>),
    Return(RStmtReturn<M>),
}

#[derive(Debug, Clone)]
pub struct RStmtLet<M> {
    pub ident: Ident, // y
    pub value: Box<RExpr<M>>, // x
}

// derive debug expression
#[derive(Debug, Clone)]
pub struct RStmtReturn<M> {
    pub value: Box<RExpr<M>>
}


// Block = (Stmt, Option<Scoped<Block>>)
// RBlock = (RStmt, Option<RExpr>)
// Everything with a scope is Scoped
// If = (Cond, Scoped<Block>, Option<Expr>)
// Expr = ... | Scoped<Block>