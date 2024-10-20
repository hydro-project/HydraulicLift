use std::ops::Deref;

use syn::{
    parse, parse_quote, Block, Expr, ExprAwait, ExprBlock, ExprIf, ExprReturn, Local, LocalInit,
    Pat, PatIdent, PatLit, Stmt,
};

use crate::{r_ast::*, utils::DebugStr};

/// Lifts syn objects into

impl From<Expr> for RExpr<()> {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Block(s) => Self::Block(s.into()),
            Expr::If(s) => Self::If(s.into()),
            s => Self::Raw(s.into()),
        }
    }
}

impl From<ExprIf> for RExprIf<()> {
    fn from(
        ExprIf {
            box cond,
            then_branch,
            else_branch,
            ..
        }: ExprIf,
    ) -> Self {
        Self {
            cond_expr: Box::new(cond.into()),
            then_expr: Box::new(RExpr::Block(then_branch.into())),
            else_expr: Box::new(
                else_branch
                    .map(|(_, box expr)| expr)
                    .unwrap_or(syn_unit())
                    .into(),
            ), // else expr or unit
        }
    }
}

impl From<ExprBlock> for RExprBlock<()> {
    fn from(expr_block: ExprBlock) -> Self {
        expr_block.block.into()
    }
}

impl From<Block> for RExprBlock<()> {
    fn from(block: Block) -> Self {
        let mut stmts = block.stmts;

        // Popped last statement if it is an expression, otherwise just ()
        let mut return_expr: RExpr<()> = match stmts.pop() {
            Some(Stmt::Expr(expr, None)) => expr.into(),
            Some(stmt) => {
                stmts.push(stmt);
                syn_unit().into()
            }
            None => syn_unit().into(),
        };

        for stmt in stmts.into_iter().rev() {
            return_expr = RExpr::Block(Self {
                stmt: stmt.into(),
                return_expr: Box::new(return_expr),
            })
        }

        // add unit stmt before to make expr block
        Self {
            stmt: RStmt::Expr(Box::new(syn_unit().into())),
            return_expr: Box::new(return_expr),
        }

        // []
        // Block { (), () }

        // [E]
        // Block { (), E }

        // [a]
        // Block { a, () }

        // [a, b, E]
        // Block { a, Block {b, E}}
    }
}

impl From<Stmt> for RStmt<()> {
    fn from(stmt: Stmt) -> Self {
        match stmt {
            Stmt::Local(Local {
                pat: Pat::Ident(PatIdent { ident, .. }),
                init:
                    Some(LocalInit {
                        box expr,
                        ..
                    }),
                ..
            }) => Self::Let(RStmtLet {
                ident,
                value: Box::new(expr.into()),
            }.into()),
            Stmt::Expr(Expr::Return(ExprReturn { expr, .. }), _) => Self::Return(RStmtReturn {
                value: Box::new(expr.map(|box e| e).unwrap_or(syn_unit()).into()),
            }),
            Stmt::Expr(expr, _) => Self::Expr(Box::new(expr.into())),
            _ => panic!("Unable to parse {:?}. This is probably not supported by Rust to Hydro yet.", stmt),
        }
    }
}

/// Returns a syn unit expr ()
fn syn_unit() -> Expr {
    parse_quote!(())
}
