use std::rc::Rc;

use crate::{
    r_ir::ir::*,
    utils::{
        debug::DebugStr,
        functional::Semigroup,
        idents::unique_ident,
        scope::{Scope, ScopeDef},
        tagged::TagOut,
    },
};

use super::{ir::*, rail::HRail};

/// Transforms an RExpr tree into a HOutput node and a collection of intermediary sinks.
pub fn generate_h(
    TagOut(expr, scope): TagOut<RExpr<ScopeDef, Scope>, Scope>,
) -> (Vec<HCycleSink>, HOutput) {
    // The overall tree will have the special cased "Hh_input" scope,
    // which will be replaced by the hydroflow+ h_input node in the next stage.
    let h_input = HScope::Input(TagOut(HInput, scope));

    let mut sinks = Vec::new();

    let output = HGen::gen(expr, h_input, &mut sinks)
        .map(HOutput::ret) // Return the final evaluated expression
        .merge(); // Merge both rails into single output

    (sinks, output)
}

/// Creates a h node (corresponding to an r node) which consumes an h_input scope.
/// Might register some cycle sinks along the way
trait HGen<H> {
    fn gen(r_node: Self, h_input: HScope, sinks: &mut Vec<HCycleSink>) -> HRail<H>;
}

impl<T, U> HGen<TagOut<U, Scope>> for TagOut<T, Scope>
where
    U: 'static,
    T: HGen<U>,
{
    fn gen(
        TagOut(inner, scope_def): Self,
        h_input: HScope,
        sinks: &mut Vec<HCycleSink>,
    ) -> HRail<TagOut<U, Scope>> {
        // Lower output tags into scopes
        HGen::gen(inner, h_input, sinks).map(|h_inner| TagOut(h_inner, scope_def))
    }
}

impl HGen<HExpr> for RExpr<ScopeDef, Scope> {
    fn gen(r_node: Self, h_input: HScope, sinks: &mut Vec<HCycleSink>) -> HRail<HExpr> {
        match r_node {
            RExpr::If(s) => HGen::gen(s, h_input, sinks),
            RExpr::Block(s) => HGen::gen(s, h_input, sinks),
            RExpr::Await(s) => HGen::gen(s, h_input, sinks),
            RExpr::Raw(s) => HGen::gen(s, h_input, sinks).map(HExpr::Raw),
        }
    }
}

impl HGen<HExprRaw> for RExprRaw<ScopeDef> {
    fn gen(
        RExprRaw {
            expr: DebugStr(expr),
            scope,
        }: Self,
        h_input: HScope,
        _: &mut Vec<HCycleSink>,
    ) -> HRail<HExprRaw> {
        HRail::pure(HExprRaw::new(expr, h_input, scope))
    }
}

impl HGen<HExpr> for RExprIf<ScopeDef, Scope> {
    fn gen(
        Self {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: Self,
        h_input: HScope,
        sinks: &mut Vec<HCycleSink>,
    ) -> HRail<HExpr> {
        HGen::gen(cond_expr, h_input, sinks).and_then(|h_cond| {
            let (then_scope, else_scope) = branch_cond(h_cond);

            let h_then = HGen::gen(then_expr, then_scope, sinks);
            let h_else = HGen::gen(else_expr, else_scope, sinks);
            h_then.concat(h_else)
        })
    }
}

impl HGen<HExpr> for RExprBlock<ScopeDef, Scope> {
    fn gen(
        Self { stmt, box expr }: Self,
        h_input: HScope,
        sinks: &mut Vec<HCycleSink>,
    ) -> HRail<HExpr> {
        HGen::gen(stmt, h_input, sinks).and_then(|h_stmt| HGen::gen(expr, h_stmt, sinks))
    }
}

impl HGen<HExpr> for RExprAwait<ScopeDef, Scope> {
    fn gen(Self(box inner): Self, h_input: HScope, sinks: &mut Vec<HCycleSink>) -> HRail<HExpr> {
        HGen::gen(inner, h_input, sinks).map(|h_inner| HExpr::Await(HExprAwait::new(h_inner)))
    }
}

impl HGen<HScope> for RStmt<ScopeDef, Scope> {
    fn gen(r_node: Self, h_input: HScope, sinks: &mut Vec<HCycleSink>) -> HRail<HScope> {
        match r_node {
            RStmt::Let(s) => HGen::gen(s, h_input, sinks).map(HScope::Bind),
            RStmt::Return(s) => HGen::gen(s, h_input, sinks),
            RStmt::While(s) => HGen::gen(s, h_input, sinks),
        }
    }
}

impl HGen<HBind> for RStmtLet<ScopeDef, Scope> {
    fn gen(
        Self {
            id,
            is_mut: _,
            box value,
        }: Self,
        h_input: HScope,
        sinks: &mut Vec<HCycleSink>,
    ) -> HRail<HBind> {
        HGen::gen(value, h_input, sinks).map(|h_value| HBind::new(id, h_value))
    }
}

impl<T: 'static> HGen<T> for RStmtReturn<ScopeDef, Scope> {
    fn gen(Self { box value }: Self, h_input: HScope, sinks: &mut Vec<HCycleSink>) -> HRail<T> {
        HGen::gen(value, h_input, sinks).and_then(|h_value| HRail::Output(HOutput::ret(h_value)))
    }
}

impl HGen<HScope> for RStmtWhile<ScopeDef, Scope> {
    fn gen(
        Self { box cond, box body }: Self,
        h_input: HScope,
        sinks: &mut Vec<HCycleSink>,
    ) -> HRail<HScope> {
        let ident = unique_ident();

        // Merge h_input with the cycle source
        let h_cycle_source = HScope::CycleSource(HCycleSource(ident.clone()));
        let h_input = h_input.concat(h_cycle_source);

        HGen::gen(cond, h_input, sinks).and_then(|h_cond| {
            let (h_then, h_exit) = branch_cond(h_cond);
            let body_out = HGen::gen(body, h_then, sinks)
                .map(|h_body| {
                    // Drop the expression the body evaluated to.
                    let h_body_scope = HScope::Expr(HDropExpr::new(h_body));
                    sinks.push(HCycleSink::new(h_body_scope, ident));
                })
                .output();

            HRail::maybe_pure(h_exit, body_out)
        })
    }
}

/// Takes a boolean condition and outputs a tuple of (then, else) scopes
fn branch_cond(h_cond: HExpr) -> (HScope, HScope) {
    // Wrap in shared so only the Rc pointer will be copied
    // when h_cond is cloned, not the underlying tree.
    let h_cond = HExpr::Shared(HExprShared::new(Rc::new(h_cond)));
    (
        HScope::Filter(HFilter::new(true, h_cond.clone())),
        HScope::Filter(HFilter::new(false, h_cond)),
    )
}
