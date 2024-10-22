use syn::Expr;

use crate::{io::IO, ir2::{ExprPat, HBlock, HExpr, HNode, HPattern, ScopePat}, r_ast::{RExpr, RExprBlock, RExprIf}, utils::{DebugStr, Tagged}};


impl From<RExpr<IO>> for Box<dyn HNode<I=ScopePat, O=dyn HPattern>> {
    fn from(value: RExpr<IO>) -> Self {
        match value {
            RExpr::If(s) => Box::new(HExpr::from(s)),
            RExpr::Block(s) => Box::new(HBlock::from(s)),
            RExpr::Raw(s) => Box::new(HExpr::from(s)),
        }
    }
}

impl From<Tagged<DebugStr<Expr>, IO>> for HExpr {
    fn from(Tagged(DebugStr(expr), IO { input_scope: scope, output_scope }): Tagged<DebugStr<Expr>, IO>) -> Self {
        Self { expr, scope }
    }
}

impl From<RExprIf<IO>> for HExpr {
    fn from(value: RExprIf<IO>) -> Self {
        todo!()
    }
}

impl<O: HPattern> From<RExprBlock<IO>> for HBlock<O> {
    fn from(RExprBlock { stmt, box return_expr }: RExprBlock<IO>) -> Self {
        let stmt = stmt.into();
        let eval = return_expr.into();
        Self { stmt, eval }
    }
}

