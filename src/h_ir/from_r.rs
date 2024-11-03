use std::rc::Rc;

use crate::{
    r_ir::ir::*,
    utils::{debug::DebugStr, functional::Semigroup, scope::Scope, tagged::Tagged},
};

use super::{ir::*, rail::HRail};

/// Transforms an RExpr tree into a HOutput node
impl From<RExpr<Scope>> for HOutput {
    fn from(expr: RExpr<Scope>) -> Self {
        // The overall tree will have the special cased "HInput" scope,
        // which will be replaced by the hydroflow+ input node in the next stage.
        let input = HScope::Input(HInput);

        expr
            .h_into(input) 
            .map(HOutput::ret) // Return the final evaluated expression
            .merge() // Merge both rails into single output
    }
}

/// Represents a transformation from T to an
/// H node which can evaluate to self or return.
trait HFrom<T>: Sized {
    /// `value` is the r node to process.
    /// `h_input` is the input scope to the entire node tree represented by `value`.
    /// Returns a rail containing nodes which evaluate `value`,
    /// in addition to nodes which evaluate early return values of `value`,
    fn h_from(value: T, h_input: HScope) -> HRail<Self>;
}

trait HInto<O: HFrom<Self>>: Sized {
    fn h_into(self, h_input: HScope) -> HRail<O>;
}

impl<T, O: HFrom<T>> HInto<O> for T {
    fn h_into(self, h_input: HScope) -> HRail<O> {
        O::h_from(self, h_input)
    }
}

impl<T, U: HFrom<T>> HFrom<Tagged<T, Scope>> for Tagged<U, Scope> {
    fn h_from(Tagged(inner, scope): Tagged<T, Scope>, h_input: HScope) -> HRail<Self> {
        inner.h_into(h_input).map(|h_inner| Tagged(h_inner, scope))
    }
}

impl HFrom<RExpr<Scope>> for HExpr {
    fn h_from(value: RExpr<Scope>, h_input: HScope) -> HRail<HExpr> {
        match value {
            RExpr::If(s) => s.h_into(h_input),
            RExpr::Block(s) => s.h_into(h_input),
            RExpr::Raw(s) => s.h_into(h_input).map(Self::Raw),
        }
    }
}

impl HFrom<RExprRaw<Scope>> for HExprRaw {
    fn h_from(
        RExprRaw {
            expr: DebugStr(expr),
            scope,
        }: RExprRaw<Scope>,
        h_input: HScope,
    ) -> HRail<Self> {
        HRail::pure(HExprRaw::new(expr, h_input, scope))
    }
}

impl HFrom<RExprIf<Scope>> for HExpr {
    fn h_from(
        RExprIf {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: RExprIf<Scope>,
        h_input: HScope,
    ) -> HRail<Self> {
        cond_expr
            .h_into(h_input)
            .map(|h_cond| HExpr::Shared(HExprShared::new(Rc::new(h_cond))))
            .and_then(|h_cond| {
                // Safe to clone here without duplicating logic because
                // only the Rc pointer will be copied, not the underlying tree.
                let h_then_cond = HScope::Filter(HFilter::new(true, h_cond.clone()));
                let h_else_cond = HScope::Filter(HFilter::new(false, h_cond));

                let h_then = then_expr.h_into(h_then_cond);
                let h_else = else_expr.h_into(h_else_cond);
                h_then.concat(h_else)
            })
    }
}

impl HFrom<RExprBlock<Scope>> for HExpr {
    fn h_from(RExprBlock { stmt, box expr }: RExprBlock<Scope>, h_input: HScope) -> HRail<HExpr> {
        stmt.h_into(h_input).and_then(|h_stmt| expr.h_into(h_stmt))
    }
}

impl HFrom<RStmt<Scope>> for HScope {
    fn h_from(value: RStmt<Scope>, h_input: HScope) -> HRail<HScope> {
        match value {
            RStmt::Let(s) => s.h_into(h_input).map(HScope::Bind),
            RStmt::Return(s) => s.h_into(h_input),
        }
    }
}

impl HFrom<RStmtLet<Scope>> for HBind {
    fn h_from(RStmtLet { id, box value }: RStmtLet<Scope>, h_input: HScope) -> HRail<Self> {
        value.h_into(h_input).map(|h_value| HBind::new(id, h_value))
    }
}

impl<T> HFrom<RStmtReturn<Scope>> for T {
    fn h_from(RStmtReturn { box value }: RStmtReturn<Scope>, h_input: HScope) -> HRail<Self> {
        value
            .h_into(h_input)
            .and_then(|h_value| HRail::Output(HOutput::ret(h_value)))
    }
}
