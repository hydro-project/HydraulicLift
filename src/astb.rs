use std::fmt::Debug;

use quote::ToTokens;
use syn::{Expr, Pat, Stmt};

/// AST B - AST A which includes 

pub enum BExpr {
    If(BExprIf),
    Block(BExprBlock),
    Raw(Expr), //TODO: expand
}

#[derive(Debug)]
pub struct BExprIf {
    pub condition: Box<BExpr>,
    pub then_block: BExprBlock,
    pub else_block: Option<Box<BExpr>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug)]
pub struct BExprBlock {
    pub statements: Vec<BStmt>,
}

pub enum BStmt {
    LetAwait(BStmtLetAwait),
    Return(BReturn),
    Expr(BExpr),
    Raw(Stmt), // TODO: expand
}

/// currently only matching let y = x.await
pub struct BStmtLetAwait {
    pub definition: Pat,    // y
    pub future: Box<BExpr>, // x
}
// derive debug expression
#[derive(Debug)]
pub struct BReturn {
    pub value: Option<BExpr>
}

fn toks_to_debug(x: impl ToTokens) -> String {
    format!("{}", x.to_token_stream())
}

impl Debug for BExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If(arg0)      => f.debug_tuple("If")      .field(arg0).finish(),
            Self::Block(arg0)   => f.debug_tuple("Block")   .field(arg0).finish(),
            Self::Raw(arg0) => f.debug_tuple("Raw").field(&toks_to_debug(arg0)).finish(),
        }
    }
}

impl Debug for BStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LetAwait(arg0)    => f.debug_tuple("LetAwait").field(arg0).finish(),
            Self::Return(arg0)      => f.debug_tuple("Return").field(arg0).finish(),
            Self::Expr(arg0)        => f.debug_tuple("Expr").field(arg0).finish(),
            Self::Raw(arg0) => f.debug_tuple("Raw").field(&toks_to_debug(arg0)).finish(),
        }
    }
}

impl Debug for BStmtLetAwait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RStmtLetAwait").field("definition", &toks_to_debug(&self.definition)).field("future", &self.future).finish()
    }
}