use std::rc::Rc;

use crate::{
    r_ir::ir::*,
    utils::{debug::DebugStr, functional::Semigroup, scope::Scope, tagged::Tagged},
};

use super::{
    ir::*,
    rail::{HRail, HRR},
};

/// Transforms an RExpr tree into a HOutput node
impl From<RExpr<Scope>> for HOutput {
    fn from(expr: RExpr<Scope>) -> Self {
        // The overall tree will have the special cased "HInput" scope,
        // which will be replaced by the hydroflow+ input node in the next stage.
        let input = HScope::Input(HInput);

        HRR::from(expr)
            .map(HOutput::ret) // Return the final evaluated expression
            .run(input)
            .merge() // Merge both rails into single output
    }
}

impl<T, U> From<Tagged<T, Scope>> for HRR<Tagged<U, Scope>>
where
    U: 'static,
    HRR<U>: From<T>,
{
    fn from(Tagged(inner, scope): Tagged<T, Scope>) -> Self {
        HRR::from(inner).map(|h_inner| Tagged(h_inner, scope))
    }
}

impl From<RExpr<Scope>> for HRR<HExpr> {
    fn from(value: RExpr<Scope>) -> Self {
        match value {
            RExpr::If(s) => HRR::from(s),
            RExpr::Block(s) => HRR::from(s),
            RExpr::Raw(s) => HRR::from(s).map(HExpr::Raw),
        }
    }
}

impl From<RExprRaw<Scope>> for HRR<HExprRaw> {
    fn from(
        RExprRaw {
            expr: DebugStr(expr),
            scope,
        }: RExprRaw<Scope>,
    ) -> Self {
        HRR::ask().map(|h_input| HExprRaw::new(expr, h_input, scope))
    }
}

impl From<RExprIf<Scope>> for HRR<HExpr> {
    fn from(
        RExprIf {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: RExprIf<Scope>,
    ) -> Self {
        HRR::from(cond_expr)
            .map(|h_cond| HExpr::Shared(HExprShared::new(Rc::new(h_cond))))
            .and_then(|h_cond| {
                // Safe to clone here without duplicating logic because
                // only the Rc pointer will be copied, not the underlying tree.
                let then_scope = HScope::Filter(HFilter::new(true, h_cond.clone()));
                let else_scope = HScope::Filter(HFilter::new(false, h_cond));

                let h_then = HRR::from(then_expr).scoped(then_scope);
                let h_else = HRR::from(else_expr).scoped(else_scope);
                h_then.concat(h_else)
            })
    }
}

impl From<RExprBlock<Scope>> for HRR<HExpr> {
    fn from(RExprBlock { stmt, box expr }: RExprBlock<Scope>) -> Self {
        HRR::from(stmt).and_then(|h_stmt| HRR::from(expr).scoped(h_stmt))
    }
}

impl From<RStmt<Scope>> for HRR<HScope> {
    fn from(value: RStmt<Scope>) -> Self {
        match value {
            RStmt::Let(s) => HRR::from(s).map(HScope::Bind),
            RStmt::Return(s) => HRR::from(s),
        }
    }
}

impl From<RStmtLet<Scope>> for HRR<HBind> {
    fn from(RStmtLet { id, box value }: RStmtLet<Scope>) -> Self {
        HRR::from(value).map(|h_value| HBind::new(id, h_value))
    }
}

impl<T: 'static> From<RStmtReturn<Scope>> for HRR<T> {
    fn from(RStmtReturn { box value }: RStmtReturn<Scope>) -> Self {
        HRR::from(value).and_then(|h_value| HRail::Output(HOutput::ret(h_value)).lift())
    }
}
