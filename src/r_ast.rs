use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Ident, Stmt};

use crate::debug_util::DebugStr;

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug)]
pub enum RExpr<M> {
    If(RExprIf<M>),
    Block(RExprBlock<M>),
    Raw(Raw<Expr, M>), //TODO: expand
}

#[derive(Debug)]
pub struct RExprIf<M> {
    pub condition: Box<RExpr<M>>,
    pub then_block: RExprBlock<M>,
    pub else_block: Option<Box<RExpr<M>>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug)]
pub struct RExprBlock<M> {
    pub statements: Vec<RStmt<M>>,
}

#[derive(Debug)]
pub enum RStmt<M> {
    LetAwait(RStmtLetAwait<M>),
    Return(RReturn<M>),
    Expr(RExpr<M>),
    Raw(Raw<Stmt, M>), // TODO: expand
}

#[derive(Debug)]
/// currently only matching let y = x.await
pub struct RStmtLetAwait<M> {
    pub definition: Ident, // y
    pub future: Box<RExpr<M>>, // x
}

// derive debug expression
#[derive(Debug)]
pub struct RReturn<M> {
    pub value: Option<RExpr<M>>
}

/// Wrapping a raw syn structure, with some metadata
#[derive(Debug)]
pub struct Raw<Syn: ToTokens, M> {
    pub syn: DebugStr<Syn>,
    pub metadata: M
}