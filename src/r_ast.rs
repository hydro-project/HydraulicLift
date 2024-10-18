use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Ident, Stmt};

use crate::utils::{DebugStr, Tagged};

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug, Clone)]
pub enum RExpr {
    If(RExprIf),
    Block(RExprBlock),
    Raw(DebugStr<Expr>), //TODO: expand
}

#[derive(Debug, Clone)]
pub struct RExprIf {
    pub condition: Box<RExpr>,
    pub then_block: RExprBlock,
    pub else_block: Option<Box<RExpr>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug, Clone)]
pub struct RExprBlock {
    pub stmt: RStmt,
    pub return_expr: Box<RExpr>
}

#[derive(Debug, Clone)]
pub enum RStmt {
    LetAwait(RStmtLetAwait),
    Return(RReturn),
    Expr(Box<RExpr>),
    Raw(DebugStr<Stmt>), // TODO: expand
}

#[derive(Debug, Clone)]
/// currently only matching let y = x.await
pub struct RStmtLetAwait {
    pub definition: Ident, // y
    pub future: Box<RExpr>, // x
}

// derive debug expression
#[derive(Debug, Clone)]
pub struct RReturn {
    pub value: Box<RExpr>
}


// Block = (Stmt, Option<Scoped<Block>>)
// RBlock = (RStmt, Option<RExpr>)
// Everything with a scope is Scoped
// If = (Cond, Scoped<Block>, Option<Expr>)
// Expr = ... | Scoped<Block>