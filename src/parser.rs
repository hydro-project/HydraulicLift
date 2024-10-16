use syn::{Block, Expr, ExprAwait, ExprBlock, ExprIf, ExprReturn, Local, LocalInit, Pat, PatIdent, Stmt};

use crate::asta::*;

impl From<Expr> for AExpr {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Block(s) => Self::Block(s.into()),
            Expr::If(s) => Self::If(s.into()),
            s => Self::Raw(s.into()),
        }
    }
}

impl From<ExprIf> for AExprIf {
    fn from(expr_if: ExprIf) -> Self {
        let ExprIf {
            box cond,
            then_branch,
            else_branch,
            ..
        } = expr_if;
        Self {
            condition: Box::new(cond.into()),
            then_block: then_branch.into(),
            else_block: else_branch.map(|(_, box expr)| Box::new(expr.into())),
        }
    }
}

impl From<ExprBlock> for AExprBlock {
    fn from(expr_block: ExprBlock) -> Self {
        expr_block.block.into()
    }
}

impl From<Block> for AExprBlock {
    fn from(block: Block) -> Self {
        Self {
            statements: block.stmts.into_iter().map(From::from).collect(), // could group_by to merge raw stmts
        }
    }
}

impl From<Stmt> for AStmt {
    fn from(stmt: Stmt) -> Self {
        match stmt {
            Stmt::Local(Local {
                pat: Pat::Ident(PatIdent {ident, ..}),
                init:
                    Some(LocalInit {
                        expr: box Expr::Await(ExprAwait { base: box base, .. }),
                        ..
                    }),
                ..
            }) => Self::LetAwait(AStmtLetAwait {
                definition: ident.into(),
                future: Box::new(base.into()),
            }),
            Stmt::Expr(Expr::Return(ExprReturn { expr, .. }), _) => Self::Return(AReturn {
                value: expr.map(|box value| value.into()),
            }),
            Stmt::Expr(expr, _) => Self::Expr(expr.into()),
            _ => Self::Raw(stmt.into()),
        }
    }
}
