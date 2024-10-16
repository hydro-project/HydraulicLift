use std::fmt::Debug;

use syn::{Expr, Ident, Stmt};

use crate::debugutil::DebugStr;

/// AST A - Extended syn AST which directly represents the Rust code.
/// New AST constructs are relevant for HF+ translation.
/// TODO: lift all HF+ AST relevant objects to the top level. All syn encapsulated objects should be raw rust

#[derive(Debug)]
pub enum AExpr {
    If(AExprIf),
    Block(AExprBlock),
    Raw(DebugStr<Expr>), //TODO: expand
}

#[derive(Debug)]
pub struct AExprIf {
    pub condition: Box<AExpr>,
    pub then_block: AExprBlock,
    pub else_block: Option<Box<AExpr>>
}

/// sequence of statements which evaluates to a value
#[derive(Debug)]
pub struct AExprBlock {
    pub statements: Vec<AStmt>,
}

#[derive(Debug)]
pub enum AStmt {
    LetAwait(AStmtLetAwait),
    Return(AReturn),
    Expr(AExpr),
    Raw(DebugStr<Stmt>), // TODO: expand
}

#[derive(Debug)]
/// currently only matching let y = x.await
pub struct AStmtLetAwait {
    pub definition: DebugStr<Ident>, // y
    pub future: Box<AExpr>, // x
}

// derive debug expression
#[derive(Debug)]
pub struct AReturn {
    pub value: Option<AExpr>
}