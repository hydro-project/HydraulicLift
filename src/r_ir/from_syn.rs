use syn::{
    parse_quote, Block, Expr, ExprAwait, ExprBlock, ExprIf, ExprReturn, Local, LocalInit, Pat, PatIdent, Stmt
};

use crate::utils::idents::ident;

use super::ir::*;
/// Lifts syn objects into

// TODO: cleanup: use R_::new constructors

impl From<Expr> for RExpr {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Block(ExprBlock { block: s, .. }) => s.into(),
            Expr::If(s) => Self::If(s.into()),
            Expr::Await(s) => Self::Await(s.into()),
            s => Self::Raw(RExprRaw::from(s).into()),
        }
    }
}

impl From<Expr> for RExprRaw {
    fn from(value: Expr) -> Self {
        Self::new(value, ())
    }
}

impl From<ExprIf> for RExprIf {
    fn from(
        ExprIf {
            box cond,
            then_branch,
            else_branch,
            ..
        }: ExprIf,
    ) -> Self {
        Self::new(
            cond.into(),
            then_branch.into(),
            else_branch
                .map(|(_, box expr)| expr)
                .unwrap_or(syn_unit())
                .into(),
        )
    }
}

impl From<Block> for RExpr {
    fn from(block: Block) -> Self {
        let mut stmts = block.stmts;

        // Popped last statement if it is an expression, otherwise just ()
        let mut return_expr = match stmts.pop() {
            Some(Stmt::Expr(expr, None)) if !matches!(expr, Expr::Return(_)) => expr.into(), //ending expression (returns should not be exprs)
            Some(stmt) => {
                // ending expression is unit if we end on a statement
                stmts.push(stmt);
                syn_unit().into()
            }
            None => syn_unit().into(), // ending expression is unit if block is empty
        };

        for stmt in stmts.into_iter().rev() {
            return_expr = Self::Block(RExprBlock::new(stmt.into(), return_expr));
        }
        return_expr
    }
}

impl From<ExprAwait> for RExprAwait {
    fn from(ExprAwait { box base, .. }: ExprAwait) -> Self {
        Self::new(base.into())
    }
}

impl From<Stmt> for RStmt {
    fn from(stmt: Stmt) -> Self {
        match stmt {
            Stmt::Local(Local {
                pat: Pat::Ident(PatIdent { ident, mutability, .. }),
                init: Some(LocalInit { box expr, .. }),
                ..
            }) => Self::Let(RStmtLet::new(ident, mutability.is_some(), expr.into()).into()),
            Stmt::Expr(Expr::Return(ExprReturn { expr, .. }), _) => Self::Return(RStmtReturn::new(
                expr.map(|box e| e).unwrap_or(syn_unit()).into(),
            )),
            Stmt::Expr(expr, _) => Self::Let(RStmtLet::new(ident("_"), false, expr.into()).into()), // expr; -> let _ = expr;
            _ => panic!(
                "Unable to parse {:?}. This is probably not supported by Rust to Hydro yet.",
                stmt
            ),
        }
    }
}

/// Returns a syn unit expr ()
fn syn_unit() -> Expr {
    parse_quote!(())
}
