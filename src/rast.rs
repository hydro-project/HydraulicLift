use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Pat, Stmt};

/// R AST - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for H AST translation.
/// TODO: lift all H AST relevant objects to the top level. All syn encapsulated objects should be raw rust

pub enum RExpr {
    If(RExprIf),
    Block(RExprBlock),
    Raw(Expr), //TODO: expand
}

#[derive(Debug)]
pub struct RExprIf {
    pub condition: Box<RExpr>,
    pub then_block: RExprBlock,
    pub else_block: Option<Box<RExpr>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug)]
pub struct RExprBlock {
    pub statements: Vec<RStmt>,
}

pub enum RStmt {
    LetAwait(RStmtLetAwait),
    Return(RReturn),
    Expr(RExpr),
    Raw(Stmt), // TODO: expand
}

/// currently only matching let y = x.await
pub struct RStmtLetAwait {
    pub definition: Pat,    // y
    pub future: Box<RExpr>, // x
}

#[derive(Debug)]
pub struct RReturn {
    pub value: Option<RExpr>
}

fn toks_to_debug(x: impl ToTokens) -> String {
    format!("{}", x.to_token_stream())
}

impl Debug for RExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If(arg0)      => f.debug_tuple("If")      .field(arg0).finish(),
            Self::Block(arg0)   => f.debug_tuple("Block")   .field(arg0).finish(),
            Self::Raw(arg0) => f.debug_tuple("Raw").field(&toks_to_debug(arg0)).finish(),
        }
    }
}

impl Debug for RStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LetAwait(arg0)    => f.debug_tuple("LetAwait").field(arg0).finish(),
            Self::Return(arg0)      => f.debug_tuple("Return").field(arg0).finish(),
            Self::Expr(arg0)        => f.debug_tuple("Expr").field(arg0).finish(),
            Self::Raw(arg0) => f.debug_tuple("Raw").field(&toks_to_debug(arg0)).finish(),
        }
    }
}

impl Debug for RStmtLetAwait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RStmtLetAwait").field("definition", &toks_to_debug(&self.definition)).field("future", &self.future).finish()
    }
}